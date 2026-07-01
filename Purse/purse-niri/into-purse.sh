#!/usr/bin/env bash
# into-purse.sh — Feed file paths to workspace's Purse instance

PID=$(purse-niri)
if [ -z "$PID" ]; then
    echo "Error: Could not get Purse PID from purse-niri" >&2
    exit 1
fi

SOCKET_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"
SOCKET="${SOCKET_DIR}/purse-${PID}.sock"

if [ ! -S "$SOCKET" ]; then
    echo "Error: Socket $SOCKET not ready" >&2
    exit 1
fi

# Canonicalize paths and write to socket
for arg in "$@"; do
    realpath -q "$arg"
done | nc -U "$SOCKET"
