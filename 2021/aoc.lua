local requests = require "http.request"
local dotenv = require "dotenv";

local env = dotenv.load()

local uri_fmt = "https://adventofcode.com/2021/day/%d/input"

local aoc = {}
-- retrieves input from Advent of Code servers
function aoc.get_input(day)
  local req = requests.new_from_uri(string.format(uri_fmt, day))
  req.headers:upsert("cookie", string.format("session=%s", env["session"]))
  local _, stream = assert(req:go())
  local body = assert(stream:get_body_as_string())
  return body
end

return aoc