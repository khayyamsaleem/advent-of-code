local fun = require "fun"
local lib = require "lib"

local solution = {}

local part_one_rules = {
  ["forward"] = function (pos, d) return { x = pos.x + d, y = pos.y     } end,
  ["down"]    = function (pos, d) return { x = pos.x,     y = pos.y + d } end,
  ["up"]      = function (pos, d) return { x = pos.x,     y = pos.y - d } end,
}

local part_two_rules = {
  ["forward"] = function (pos, d) return { x = pos.x + d, y = pos.y + (pos.aim * d), aim = pos.aim     } end,
  ["down"]    = function (pos, d) return { x = pos.x,     y = pos.y,                 aim = pos.aim + d } end,
  ["up"]      = function (pos, d) return { x = pos.x,     y = pos.y,                 aim = pos.aim - d } end,
}

function solution.parse_input(input)
  return fun.map(
    function (cur)
      local _, _, direction, steps = string.find(cur, "(%a+)%s*(%d+)")
      return { direction = direction, steps = tonumber(steps) }
    end,
    lib.iter_hack(input:gmatch("([^\n]*)\n?"))
  )
end

function solution.move_submarine(submarine, action, rules)
  rules = rules or part_one_rules
  return rules[action.direction](submarine, action.steps)
end

function solution.follow_directions(directions, submarine, rules)
  rules = rules or part_one_rules
  return fun.reduce(
    function (acc, cur)
      return solution.move_submarine(acc, cur, rules)
    end,
    submarine,
    directions
  )
end

function solution.partOne(input, submarine)
  submarine = submarine or { x = 0, y = 0 }
  local final_position = solution.follow_directions(solution.parse_input(input), submarine)
  return final_position.x * final_position.y
end

function solution.partTwo(input, submarine)
  submarine = submarine or { x = 0, y = 0, aim = 0 }
  local final_position = solution.follow_directions(solution.parse_input(input), submarine, part_two_rules)
  return final_position.x * final_position.y
end

return solution