local aoc = require "aoc"
local dotenv = require "dotenv"

local solution = require "02.solution"

local env = dotenv.load()

local input = aoc.get_input("https://adventofcode.com/2021/day/2/input", env["session"])

print("Part One: ", solution.partOne(input))
print("Part Two: ", solution.partTwo(input))