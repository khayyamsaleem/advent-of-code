local requests = require "http.request"

local aoc = {}
-- retrieves input from Advent of Code servers
function aoc.get_input(uri, token)
  local req = requests.new_from_uri(uri)
  req.headers:upsert("cookie", string.format("session=%s", token))
  local headers, stream = assert(req:go())
  local body = assert(stream:get_body_as_string())
  return body
end

return aoc