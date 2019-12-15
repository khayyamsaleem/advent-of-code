local solution = require "07"
local dotenv = require "dotenv"
 
local env = dotenv.load()

program = {3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0}
phase_setting = {0,1,2,3,4}
assert(solution.amplifier_runthrough(env["intcode_server"], program, phase_setting) == 54321)

program = {3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0}
phase_setting = {1,0,4,3,2}
assert(solution.amplifier_runthrough(env["intcode_server"], program, phase_setting) == 65210)

program = {3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0}
phase_setting = {4,3,2,1,0}
assert(solution.amplifier_runthrough(env["intcode_server"], program, phase_setting) == 43210)
