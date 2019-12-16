local dotenv = require "dotenv"
local requests = require "http.request"
local inspect = require "inspect"
local C = require "combine"
local Amp = require "amp"
local IntcodeService = require "intcode-service"

local solution = {}

-- retrieves input from Advent of Code servers
function get_input(uri, token)
    local req = requests.new_from_uri(uri)
    req.headers:upsert("cookie", string.format("session=%s", token))
    local headers, stream = assert(req:go())
    local body = assert(stream:get_body_as_string())
    local t={}
    for s in string.gmatch(string.gsub(body, "%s+", ""), "([^,]+)") do
        table.insert(t, tonumber(s))
    end
    return t
end

-- poor but functioning part One solution, written before I created the amp class
function solution.amplifier_runthrough(intcode_service, software, phase_settings)
    local next_input = {0}
    for _,phase_setting in pairs(phase_settings) do
        next_input = intcode_service:eval(software, {phase_setting, table.unpack(next_input)})["output-signals"]
    end
    return next_input[ #next_input ]
end

-- quick max-of-table function with comparator
function max(t, fn)
    if #t == 0 then return nil, nil end
    local key, value = 1, t[1]
    for i = 2, #t do
        if fn(value, t[i]) then
            key, value = i, t[i]
        end
    end
    return key, value
end

-- part one solution
function solution.partOne()
    local env = dotenv.load()
    local program = get_input("https://adventofcode.com/2019/day/7/input", env["session"])
    local intcode_service = IntcodeService:new(env["intcode_server"])
    local phase_options = {0,1,2,3,4}
    local signals = {}
    -- iterates over all combinations of the phase_options
    for phase_setting in C.permute(phase_options) do
        signal = solution.amplifier_runthrough(intcode_service, program, phase_setting)
        table.insert(signals, signal)
    end
    local loc, maxVal = max(signals, function(a,b) return a < b end)
    return maxVal
end

-- part two solution
function solution.partTwo()
    local env = dotenv.load()
    local program = get_input("https://adventofcode.com/2019/day/7/input", env["session"])
    local intcode_service = IntcodeService:new(env["intcode_server"])
    local signals = {}
    local AMP_NAMES = {"A", "B", "C", "D", "E"}
    for phase_setting in C.permute({5,6,7,8,9}) do
        -- creates "ring" of amps
        local amps = Amp.sequence_from_list(intcode_service, AMP_NAMES, program)
        -- executes amp ring for a given phase setting
        amps = Amp.run_sequence(amps, phase_setting)
        -- retrieves last output signal from last amp and stores it
        local output_signals = amps[#amps].output_signals
        table.insert(signals, output_signals[#output_signals])
    end
    local loc, maxVal = max(signals, function (a,b) return a < b end)
    return maxVal
end

return solution
