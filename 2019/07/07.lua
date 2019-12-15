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
    local bodyJson = json.decode(body)
    return bodyJson
end

function resume_eval_by_pid(intcode_server_uri, pid, inputs)
    local req = requests.new_from_uri(intcode_server_uri .. "/eval")
    req.headers:upsert(":method", "POST")
    req:set_body(json.encode({ pid = pid, inputs = inputs}))
    local headers, stream = assert(req:go())
    local body = assert(stream:get_body_as_string())
    local bodyJson = json.decode(body)
    return bodyJson
end

function solution.amplifier_runthrough(intcode_server_uri, software, phase_settings)
    local next_input = {0}
    for _,phase_setting in pairs(phase_settings) do
        next_input = eval_intcode(intcode_server_uri, software, {phase_setting, table.unpack(next_input)})["output-signals"]
    end
    return next_input[ #next_input ]
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
    local k, v = max(signals, function(a,b) return a < b end)
    return v
end

function solution.partTwo()
    env = dotenv.load()
    -- local program = get_input("https://adventofcode.com/2019/day/7/input", env["session"])
    local phase_options = {9,8,7,6,5}
    local program = {3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5}
    local amp_pids = {A = nil, B = nil, C = nil, D = nil, E = nil}
    local amps = {"A", "B", "C", "D", "E"}
    local phase_setting_ptr = 1
    local input_buffer = {phase_options[phase_setting_ptr], 0}
    print(inspect(input_buffer))
    repeat
        for _,amp in pairs(amps) do
            pid = amp_pids[amp]
            local res = nil
            if (pid ~= nil)
            then
                res = resume_eval_by_pid(env["intcode_server"], pid, { input_buffer[1], input_buffer[2] })
            else
                res = eval_intcode(env["intcode_server"], program, { input_buffer[1], input_buffer[2]})
            end
            table.remove(input_buffer, 1)
            table.remove(input_buffer, 2)
            if (res["pid"] ~= nil)
            then
                print("Pausing execution for " .. amp)
                print("PID: " .. res["pid"])
                amp_pids[amp] = res["pid"]
                for _,v in pairs(res["output-signals"]) do
                    table.insert(input_buffer, v)
                end
                if (phase_setting_ptr < #phase_options)
                then
                    phase_setting_ptr = phase_setting_ptr + 1
                    table.insert(input_buffer, phase_options[phase_setting_ptr])
                end
            else
                print(amp .. " HALTED WITH " .. inspect(res["output-signals"]))
                amp_pids[amp] = nil
            end
        end
    until (amp_pids["E"] == nil)
end

return solution
