local aoc = require "aoc_lib";
local solution = require "01";
local dotenv = require "dotenv";

local env = dotenv.load()

local input = aoc.get_input("https://adventofcode.com/2021/day/1/input", env["session"])

local window_length = 3

print("Part One: ", solution.partOne(input))
print("Part Two: ", solution.partTwo(input, window_length))