require 'busted.runner'()
local fun = require "fun"
local inspect = require "inspect"

local solution = require "02.solution"

local test_input = [[forward 5
down 5
forward 8
up 3
down 8
forward 2]]

local test_rules = {
  ["forward"] = function (pos, d) return { x = pos.x + d, y = pos.y + (pos.aim * d), aim = pos.aim     } end,
  ["down"]    = function (pos, d) return { x = pos.x,     y = pos.y,                 aim = pos.aim + d } end,
  ["up"]      = function (pos, d) return { x = pos.x,     y = pos.y,                 aim = pos.aim - d } end,
}

describe("parse_input", function ()
  it("creates direction objects from input", function ()
    local directions = solution.parse_input(test_input)
    local first = fun.head(directions)
    assert.are.equal(first.direction, "forward")
    assert.are.equal(first.steps, 5)
  end)
end)

describe("move_submarine", function ()
  it("moves the submarine in accordance with the action", function()
    local sub = { x = 0, y = 0 }
    local new_position = solution.move_submarine(sub, { direction = "forward", steps = 5 })
    assert.are.equal(new_position.x, 5)
    assert.are.equal(new_position.y, 0)
    local next_position = solution.move_submarine(new_position, { direction = "down", steps = 5 })
    assert.are.equal(next_position.x, 5)
    assert.are.equal(next_position.y, 5)
  end)
end)

describe("follow directions", function ()
  it("moves the submarine according to the instructions", function()
    local sub = { x = 0, y = 0 }
    local directions = solution.parse_input(test_input)
    local final_position = solution.follow_directions(directions, sub)
    assert.are.equal(final_position.x, 15);
    assert.are.equal(final_position.y, 10);
  end)

  it("moves the submarine according to different instructions", function()
    local sub = { x = 0, y = 0, aim = 0 }
    local directions = solution.parse_input(test_input)
    local final_position = solution.follow_directions(directions, sub, test_rules)
    assert.are.equal(final_position.x, 15);
    assert.are.equal(final_position.y, 60);
  end)
end)

describe("part one", function ()
  it("yields the correct answer", function ()
    local sub = { x = 0, y = 0 }
    assert.are.equal(solution.partOne(test_input, sub), 150)
  end)
end)

describe("part two", function ()
  it("yields the correct answer", function ()
    local sub = { x = 0, y = 0, aim = 0 }
    assert.are.equal(solution.partTwo(test_input, sub), 900)
  end)
end)