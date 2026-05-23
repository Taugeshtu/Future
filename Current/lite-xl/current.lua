-- mod-version:3
local core = require "core"
local Doc = require "core.doc"

-- Get the exact editor process PID using FFI (with PPID fallback)
local pid = nil
local has_ffi, ffi = pcall(require, "ffi")
if has_ffi then
  pcall(function()
    ffi.cdef("int getpid(void);")
    pid = ffi.C.getpid()
  end)
end
if not pid then
  local fp = io.popen("sh -c 'echo $PPID'")
  if fp then
    pid = tonumber(fp:read("*a"))
    fp:close()
  end
end

local runtime_dir = os.getenv("XDG_RUNTIME_DIR") or "/run/user/1000"
local socket_path = runtime_dir .. "/current.sock"

-- Lightweight JSON Encoder
local function json_encode(val)
  if type(val) == "string" then
    return string.format("%q", val)
  elseif type(val) == "number" then
    return tostring(val)
  elseif type(val) == "boolean" then
    return val and "true" or "false"
  elseif type(val) == "table" then
    local is_array = true
    local max_idx = 0
    for k, _ in pairs(val) do
      if type(k) ~= "number" or k < 1 or math.floor(k) ~= k then
        is_array = false
        break
      end
      if k > max_idx then max_idx = k end
    end
    if is_array then
      local parts = {}
      for i = 1, max_idx do
        table.insert(parts, json_encode(val[i]))
      end
      return "[" .. table.concat(parts, ",") .. "]"
    else
      local parts = {}
      for k, v in pairs(val) do
        table.insert(parts, string.format("%q:%s", k, json_encode(v)))
      end
      return "{" .. table.concat(parts, ",") .. "}"
    end
  else
    return "null"
  end
end

-- Publish Context Function
local function publish_context()
  local view = core.active_view
  local DocView = require "core.docview"
  if not view or not view:is(DocView) then
    return
  end
  local doc = view.doc
  if not doc then
    return
  end
  local file_path = doc.abs_filename or doc.filename
  if not file_path then
    return
  end

  local selections = {}
  for idx, line, col, anchor_line, anchor_col in doc:get_selections() do
    table.insert(selections, {
      line = line,
      column = col,
      anchor_line = anchor_line,
      anchor_column = anchor_col
    })
  end

  local payload = {
    type = "Publish",
    pid = pid or 0,
    attention = {
      file = file_path,
      selections = selections
    }
  }

  local json_str = json_encode(payload)
  -- Use sh -c 'echo "payload" | nc -U "socket"' so the pipe is parsed correctly
  local cmd = string.format("sh -c 'echo %q | nc -U %q >/dev/null 2>&1'", json_str, socket_path)
  system.exec(cmd)
end

-- Throttling thread (wakes up every 100ms to publish if needed)
local needs_publish = true -- Publish on startup
core.add_thread(function()
  while true do
    if needs_publish then
      pcall(publish_context)
      needs_publish = false
    end
    coroutine.yield(0.1) -- Throttle to max 10 updates per second
  end
end)

-- Hook: Selection/Cursor changes
local original_set_selections = Doc.set_selections
function Doc:set_selections(...)
  original_set_selections(self, ...)
  needs_publish = true
end

-- Hook: View/Tab changes
local original_set_active_view = core.set_active_view
function core.set_active_view(view)
  original_set_active_view(view)
  needs_publish = true
end

-- Hook: Window Title (appends PID for daemon parsing)
local original_set_window_title = system.set_window_title
function system.set_window_title(title)
  local custom_title = string.format("%s [PID: %d]", title, pid or 0)
  original_set_window_title(custom_title)
end
