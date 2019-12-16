local requests = require "http.request"
local json = require "rapidjson"
local inspect = require "inspect"

local IntcodeService = {}
IntcodeService.__index = IntcodeService

-- creates an instance of an intcode service client given the URI for the server
function IntcodeService:new(server_uri)
    local svc = {}
    setmetatable(svc,IntcodeService)
    svc.server_uri = server_uri
    return svc
end

-- sends the eval call for a program, its input queue, and where the evaluation should begin
function IntcodeService:eval(program, inputs, program_counter)
    program_counter = program_counter or 0 -- idiomatic lua optional parameter
    print("Sending inputs: " .. inspect(inputs) .. 
          " and pos: " .. program_counter .. " to intcode service")
    local req = requests.new_from_uri(self.server_uri .. "/eval")
    req.headers:upsert(":method", "POST")
    req:set_body(json.encode({
        program = program,
        inputs = inputs,
        program_counter = program_counter
    }))
    local headers, stream = nil
    -- retry logic bc even 20 round-robin racket servelets can't take the heat
    local f = function() headers, stream = assert(req:go(2)) end
    while (not pcall(f)) do
        print("retying eval")
    end

    local body = assert(stream:get_body_as_string())
    print("got response from intcode-server")
    local bodyJson = json.decode(body)
    return bodyJson
end

return IntcodeService