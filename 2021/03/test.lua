require 'busted.runner'()

local solution = require "03.solution"
local fun = require "fun"

local test_input = [[00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010]]

describe("parse input", function ()
  it("parses input to a diagnostic report", function ()
    local diagnostic_report = solution.parse_input(test_input)
    assert.are.equal(fun.length(diagnostic_report), 12)
  end)
end)