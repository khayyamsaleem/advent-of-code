local fun = require "fun";
local lib = require "lib";

local solution = {}

function solution.parse_input(input)
  return fun.map(
    function(row)
      return fun.map(tonumber, lib.iter_hack(row:gmatch(".")))
    end,
    lib.iter_hack(input:gmatch("([^\n]*)\n?"))
  )
end

function solution.compute_rates(diagnostic_report)
  local height = 0
  local t = fun.reduce(
    function(acc, row)
      height = height + 1
      return fun.map(function (x, y) return x + y end, fun.zip(row, acc))
    end,
    fun.zeros(),
    diagnostic_report
  )
  local rates = fun.reduce(
    function (acc, num_ones)
      if num_ones > height / 2 then
        return { gamma = acc.gamma .. 1, epsilon = acc.epsilon .. 0 }
      else
        return { gamma = acc.gamma .. 0, epsilon = acc.epsilon .. 1 }
      end
    end,
    { gamma = "", epsilon = "" },
    t
  )
  return rates
end


function solution.partOne(input)
  local diagnostic_report = solution.parse_input(input)
  local rates = solution.compute_rates(diagnostic_report)
  return tonumber(rates.gamma, 2) * tonumber(rates.epsilon, 2)
end


return solution