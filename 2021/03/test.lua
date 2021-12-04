require 'busted.runner'()
local inspect = require "inspect";

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

describe("compute rates", function ()
  it("computes correct gamma and epsilon", function ()
    local rates = solution.compute_rates(solution.parse_input(test_input))
    assert.are.equal(rates.gamma, "10110")
    assert.are.equal(rates.epsilon, "01001")
  end)
end)