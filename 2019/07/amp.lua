Amp = {}
Amp.__index =  {}

function Amp:create(name, prg) {
    local amp = {}
    setmetatable(amp, Amp)
    amp.name = name
    amp.next = nil
    amp.prg = {}
    amp.pointer = 0
    amp.inputs = {}
    amp.halted = false
    return amp
}

Amp:next() {
    return amp.next
}

Amp:run() {

}