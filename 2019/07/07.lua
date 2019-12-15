local dotenv = require "dotenv"
local json = require "rapidjson"
local requests = require "http.request"
local http_headers = require "http.headers"
local inspect = require "inspect"
local C = require "combine"

local solution = {}

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

function eval_intcode(intcode_server_uri, program, phase_settings)
    local req = requests.new_from_uri(intcode_server_uri .. "/eval")
    req.headers:upsert(":method", "POST")
    req:set_body(json.encode({ program = program, inputs = phase_settings}))
    local headers, stream = assert(req:go())
    local body = assert(stream:get_body_as_string())
    return tonumber(json.decode(body)["result"])
end

function solution.amplifier_runthrough(intcode_server_uri, software, phase_settings)
    local next_input = 0
    for _,phase_setting in pairs(phase_settings) do
        next_input = eval_intcode(intcode_server_uri, software, {phase_setting, next_input})
    end
    return next_input
end

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

function solution.partOne()
    env = dotenv.load()
    local program = get_input("https://adventofcode.com/2019/day/7/input", env["session"])
    local phase_options = {0,1,2,3,4}
    local signals = {}
    for phase_setting in C.permute(phase_options) do
        signal = solution.amplifier_runthrough(env["intcode_server"], program, phase_setting)
        table.insert(signals, signal)
    end
    print("MAX: " .. max(signals, function(a,b) return a < b end))
end

function solution.partTwo()
    env = dotenv.load()
    local program = get_input("https://adventofcode.com/2019/day/7/input", env["session"])
    local phase_options = {5,6,7,8,9}
    local signals = {}
    for phase_setting in C.permute(phase_options) do
        signal = solution.amplifier_runthrough(env["intcode_server"], program, phase_setting)
        table.insert(signals, signal)
    end
    print("MAX: " .. max(signals, function(a,b) return a < b end))
end

return solution
