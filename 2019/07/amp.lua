local inspect = require "inspect"

-- lua "magic" to create a class-like structure out of a table
Amp = {}
Amp.__index =  Amp


-- instantiates a new "amp" object
function Amp:create(intcode_service, name, prg)
    local amp = {}
    setmetatable(amp, Amp)
    amp.name = name
    amp.next = nil
    amp.prg = prg
    amp.pointer = 0
    amp.inputs = {}
    amp.halted = false
    amp.output_signals = {}
    amp.intcode_service = intcode_service
    amp.halt_value = nil
    amp.blocked = false
    return amp
end

-- linked-list style amp setNext
-- I don't know if I'm actually using this part of the
-- data structure in an effective way, as I've still got a
-- dependency on the indexed table underneath
-- are circular linked lists usually indexed? I suppose 
-- I'd use the "name" of the amp to see if the last amp stopped
function Amp:setNext(next)
    self.next = next
end

-- linked-list style amp getNext
function Amp:getNext()
    return self.next
end

-- adds any amount of inputs to the input-buffer
function Amp:offerInput(...)
    for _,v in pairs({...}) do
        table.insert(self.inputs, v)
    end
end

-- returns true if program reached opcode 99
function Amp:isHalted()
    return self.halted
end

-- returns true if program tried to getInput and the queue was empty
function Amp:isBlocked()
    return self.blocked
end

-- executes a program for an amp
function Amp:run()
    -- if program already reached opcode 99, do not execute
    -- I think the input was designed so that this doesn't happen
    if (self:isHalted())
    then
        return self
    end

    -- I'm not sure if I'm doing the right thing here. Some chatter on the subreddit led me to
    -- believe that I need to halt on every call to opcode 3, so I did this. But I am skeptical,
    -- I think that there should be a way to consume whatever is on the input queue. This would
    -- reduce the number of intcode server requests that I have to make by quite a bit
    local response = self.intcode_service:eval(self.prg, { self.inputs[1] }, self.pointer)

    -- shitty logic to consume the first input off
    table.remove(self.inputs, 1)

    -- halt scenario
    -- flag amp as halted and store output signals and final day 1-esque value
    if (response["success"] ~= nil)
    then
        if (response["success"] ~= true)
        then
            print("AMP " .. self.name .. " execution failed")
            return nil
        end
        self.blocked = false
        self.halted = true
        self.output_signals = response["output-signals"]
        self.halt_value = response["result"]
    end

    -- scenario where program is waiting on input
    -- flag amp as blocked
    -- API returns program execution state, saved to the amp object
    if (response["blocked"] ~= nil)
    then
        self.blocked = true
        self.output_signals = response["output-signals"]
        self.prg = response["program"]
        self.pointer = response["program-counter"]
    end
    return self
end

-- reset output signals, for use after they have been passed to the next amp in the ring
function Amp:clearOutputSignals()
    self.output_signals = {}
end

-- static module method to generate a "ring" of amps
function Amp.sequence_from_list(intcode_service, amp_names, program)
    local amps = {}
    for _,name in pairs(amp_names) do
        table.insert(amps, Amp:create(intcode_service, name, program))
    end
    for i,amp in pairs(amps) do
        amp:setNext(amps[i+1])
    end
    amps[#amps]:setNext(amps[1])
    return amps
end

-- static module method to run a "ring" of amps until the last one halts
function Amp.run_sequence(amps, phase_settings)
    -- accounting for the start scenario, where the first non-phase-setting input signal is zero
    amps[1]:offerInput(phase_settings[1], 0)
    local i = 1
    repeat
        -- makes sure that no output signals are consumed more than once
        amps[i]:clearOutputSignals()

        -- executes software on the current amp and updates its state accordingly
        amps[i] = amps[i]:run()

        -- consume off the phase setting that is only valid for the first loop through the amp ring
        if (phase_settings[1] ~= nil)
        then
            table.remove(phase_settings, 1)
        end

        -- if there are phase settings left, we're still in the first lap through the amps, so it
        -- must be added to the front of the input queue for the next amp
        if (phase_settings[1] ~= nil)
        then
            amps[i]:getNext():offerInput(phase_settings[1])
        end

        -- add all the output signals to the input queue of the next amp
        amps[i]:getNext():offerInput(table.unpack(amps[i].output_signals))

        -- could have used modulus if lua did zero-indexed tables but oh well
        if (i == #amps)
        then
            i = 1
        else
            i = i + 1
        end
    until (amps[#amps]:isHalted()) -- repeat process until last amp is halted
    return amps
end

function Amp:__tostring()
    return "AMP " .. self.name .. ": {" ..
           "\n\tprogram = " .. inspect(self.prg):sub(0,30) .. "... }" ..
           "\n\tinputs  = " .. inspect(self.inputs) ..
           "\n\tpointer = " .. self.pointer ..
           "\n\thalted  = " .. tostring(self:isHalted()) ..
           "\n\tblocked = " .. tostring(self:isBlocked()) ..
           "\n\toutput  = " .. inspect(self.output_signals[1]) ..
           "\n}"
end

return Amp