#!/usr/bin/env bash
set -e

XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"

# 1. Query Current for cursor context (MUST do while editor is focused)
CONTEXT=$(echo '{"type": "Query"}' | nc -U "$XDG_RUNTIME_DIR/current.sock")

FILE=$(echo "$CONTEXT" | jq -r '.attention.file')
LINE=$(echo "$CONTEXT" | jq -r '.attention.selections[0].line')
COL=$(echo "$CONTEXT" | jq -r '.attention.selections[0].column')

if [ -z "$FILE" ] || [ "$FILE" = "null" ]; then
    echo "No active file in Current context" >&2
    exit 1
fi

# 2. Get active Purse PID (spawn window immediately for visual feedback)
PID=$(purse-niri)
SOCKET="$XDG_RUNTIME_DIR/purse-${PID}.sock"

# 3. Query lsp-broker (blocking)
LSP_LINE=$((LINE - 1))
LSP_COL=$((COL - 1))

REQ_BODY=$(jq -n \
  --arg file "file://$FILE" \
  --argjson line "$LSP_LINE" \
  --argjson col "$LSP_COL" \
  '{
     jsonrpc: "2.0",
     id: 1,
     method: "textDocument/definition",
     params: {
       textDocument: { uri: $file },
       position: { line: $line, character: $col }
     }
   }')

LEN=${#REQ_BODY}
REQ_PAYLOAD="Content-Length: $LEN\r\n\r\n$REQ_BODY"

RESPONSE=$(echo -ne "$REQ_PAYLOAD" | nc -w 10 -U "$XDG_RUNTIME_DIR/lsp-broker-query.sock" || true)

if [ -z "$RESPONSE" ]; then
    echo "LSP query failed or timed out" >&2
    exit 1
fi

JSON_BODY=$(echo "$RESPONSE" | sed -n '1,/^\r*$/!p')

# 4. Parse locations
LOCATIONS=$(echo "$JSON_BODY" | python3 -c '
import sys, json, urllib.parse
try:
    data = json.load(sys.stdin)
except Exception:
    sys.exit(0)
res = data.get("result")
if not res:
    sys.exit(0)
if isinstance(res, dict):
    res = [res]
for loc in res:
    uri = loc.get("uri") or loc.get("targetUri", "")
    path = urllib.parse.unquote(uri.replace("file://", ""))
    range_val = loc.get("range") or loc.get("targetSelectionRange") or loc.get("targetRange") or {}
    start = range_val.get("start", {})
    line = start.get("line", 0) + 1
    col = start.get("character", 0) + 1
    print(f"{path}:{line}:{col}")
' || true)

if [ -z "$LOCATIONS" ]; then
    echo "No definitions found" >&2
    exit 0
fi

# 5. Catch: check if window socket still exists before sending
if [ -S "$SOCKET" ]; then
    echo "$LOCATIONS" | nc -U "$SOCKET"
else
    echo "Purse window closed, discarding results" >&2
fi
