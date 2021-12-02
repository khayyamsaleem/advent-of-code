require 'busted.runner'()
local solution = require "01.solution"

local test_input = [[199
200
208
210
200
207
240
269
260
263]]
local window_length = 3

describe("part one is correct", function ()
 it("should count number of increases", function ()
  assert.are.equal(solution.partOne(test_input), 7)
 end)
end)

describe("part two is correct", function ()
 it("should count number of increases over a provided sliding window", function ()
  assert.are.equal(solution.partTwo(test_input, window_length), 5)
 end)
end)
