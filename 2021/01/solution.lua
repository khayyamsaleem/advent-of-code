local fun = require "fun"

local solution = {}

local function parse_input(input)
  local t = {}
  for m in input:gmatch("([^\n]*)\n?") do
      table.insert(t, tonumber(m))
  end
  return t
end

function solution.partOne(input)
  local measurements = parse_input(input)
  local increases = 0
  for i, _ in pairs(measurements) do
    if measurements[i+1] == nil then break end
    if measurements[i] < measurements[i+1] then
      increases = increases + 1
    end
  end
  return increases
end

function solution.partTwo(input, window_length)
  local measurements = fun.iter(parse_input(input))
  local increases = 0
  while fun.nth(window_length, measurements) ~= nil do
    local rest = fun.tail(measurements)
    local A = fun.take(window_length, measurements)
    local B = fun.take(window_length, rest)
    if fun.sum(B) > fun.sum(A) then increases = increases + 1 end
    measurements = rest
  end
  return increases
end

return solution