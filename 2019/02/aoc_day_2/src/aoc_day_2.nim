import httpClient
import dotenv
import strformat
import strutils
import os
import unpack
from sequtils import map
import sugar

# Getting secrets from .env file
let env = initDotEnv()
env.load()

# enum to represent the different operations
type
  Opcodes = enum
    plus = 1, mul = 2, halt = 99

proc getInputByDay(day: int): seq[int] =
  # storing cookie
  let sessionCookie = getEnv("session")

  # adding cookie to request headers
  var client = newHttpClient()
  client.headers = newHttpHeaders({"Cookie": &"session={sessionCookie}"})

  # retrieving input
  let response = client.get(&"https://adventofcode.com/2019/day/{day}/input")
  result = response.body.strip.split(',').map(parseInt)

# just the standard opcode processor
proc partOne(inp: seq[int]) : int =
  var output = inp
  var i = 0
  block process:
    while i <= output.len:
      case Opcodes(output[i]):
        of plus:
          [a, b] <- output[(i+1)..(i+2)].map((pos) => output[pos])
          output[output[i+3]] = a + b
          i = i + 4
        of mul:
          [a, b] <- output[(i+1)..(i+2)].map((pos) => output[pos])
          output[output[i+3]] = a * b
          i = i + 4
        of halt:
          break process
  result = output[0]

proc partTwo(inp: seq[int], target: int) : int =
  for noun in 1..99:
    for verb in 1..99:
      var test = inp
      test[1] = noun
      test[2] = verb
      if partOne(test) == target:
        return 100 * noun + verb

when isMainModule:
  var input = getInputByDay(2)
  case paramStr(1):
    of "1":
      input[1] = 12
      input[2] = 2
      echo partOne(input)
    of "2":
      let target = parseInt(paramStr(2))
      echo partTwo(input, target)
    else:
      echo "invalid input"
