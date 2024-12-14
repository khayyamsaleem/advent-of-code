import strutils

type Grid* = seq[seq[char]]

proc grid(input: string): Grid =
  var g: seq[seq[char]] = @[]
  for l in input.splitWhitespace():
    g.add(@l)
  g

const target: array[4, char] = ['X','M','A','S'];

proc find(grid: Grid, target: string): int =
  0


when compileOption("app","lib"):
  {.push,dynlib.}
proc solve(input: cstring) {.cdecl, exportc.} =
  let result = input.len
  echo "length: ", result
  echo grid($input)


