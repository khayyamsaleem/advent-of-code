local aoc = require "aoc";
local solution = require "01.solution";


local input = aoc.get_input(1)

local window_length = 3

print("Part One: ", solution.partOne(input))
print("Part Two: ", solution.partTwo(input, window_length))