local fun = require "fun";
local lib = require "lib";
local inspect = require "inspect"

local solution = {}

function solution.parse_input(input)
  return fun.map(
    function(row)
      return fun.map(tonumber, lib.iter_hack(row:gmatch(".")))
    end,
    lib.iter_hack(input:gmatch("([^\n]*)\n?"))
  )
end

local function get_bit_counts(diagnostic_report)
  local height = 0
  local t = fun.reduce(
    function(acc, row)
      height = height + 1
      return fun.map(fun.operator.add, fun.zip(row, acc))
    end,
    fun.zeros(),
    diagnostic_report
  )
  return height, t
end

function solution.compute_rates(diagnostic_report)
  local height, t = get_bit_counts(diagnostic_report)
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

local function gen_to_table(gen)
  local diagnostic_report = fun.reduce(
    function (acc, cur)
      table.insert( acc, fun.reduce( function (t, bit) table.insert(t, bit); return t end, {}, cur))
      return acc
    end,
    {},
    gen
  )
  return diagnostic_report
end

local function get_active_bits(diagnostic_report, col_sums)
  local t = fun.reduce(
    function (acc, cur)
      local head = fun.car(cur)
      local tail = fun.cdr(cur)
      table.insert(acc.next, tail)
      return { sum = acc.sum + head, next = acc.next }
    end,
    { sum = 0, next = {} },
    diagnostic_report
  )
  table.insert(col_sums, t.sum)
  if fun.length(fun.car(t.next)) == 0 then return col_sums end
  return solution.get_active_bits(t.next, col_sums)
end

function solution.compute_o2_rating(diagnostic_report, i)
  i = i or 1
  local height, bit_counts = get_bit_counts(diagnostic_report)
  local winner = 0
  if fun.nth(i, bit_counts) >= height/2 then winner = 1 end
  local t = fun.filter(
    function (bits)
      if fun.nth(i, bits) == winner then return true else return false end
    end,
    diagnostic_report
  )
  if fun.length(t) == 1 then return fun.reduce(fun.operator.concat, "", fun.car(t)) end
  return solution.compute_o2_rating(t, i+1)
end

function solution.compute_co2_rating(diagnostic_report, i)
  i = i or 1
  local height, bit_counts = get_bit_counts(diagnostic_report)
  local winner = 1
  if fun.nth(i, bit_counts) >= height/2 then winner = 0 end
  local t = fun.filter(
    function (bits)
      if fun.nth(i, bits) == winner then return true else return false end
    end,
    diagnostic_report
  )
  if fun.length(t) == 1 then return fun.reduce(fun.operator.concat, "", fun.car(t)) end
  return solution.compute_co2_rating(t, i+1)
end

function solution.partTwo(input)
  local diagnostic_report = gen_to_table(solution.parse_input(input))
  local o2 = solution.compute_o2_rating(diagnostic_report)
  local co2 = solution.compute_co2_rating(diagnostic_report)
  return tonumber(o2, 2) * tonumber(co2, 2)
end

return solution