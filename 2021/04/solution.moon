import reduce, operator from require "fun"
import fold from require "moon"
import inspect from require "inspect"

class BingoGame
  new: (choices, boards) =>
    @choices = choices
    @boards = boards
  
  play: =>
    for choice in *@choices
      for i,_ in pairs(@boards)
        @boards[i]\mark(choice)
        if @boards[i]\check! then
          return choice, @boards[i], i
    return nil

class Board
  new: (input) =>
    @board = [ [tonumber(x) for x in string.gmatch(row, "%S+")] for row in string.gmatch(input, "([^\n]*)\n?") ]

  mark: (choice) =>
    @board = [ [(if x == choice then "X" else x) for x in *row] for row in *@board ]
  
  check: =>
    for j, row in pairs(@board)
      for i, item in pairs(row)
        if @board[i][j] == "X" then
          found = true
          for c = 1, 4
            if j + c <= 5 and @board[i][j+c] ~= "X" then found = false
            if j + c > 5 then found = false
          if found == false then
            found = true
            for c = 1, 4
              if i + c <= 5 and @board[i+c][j] ~= "X" then found = false
              if i + c > 5 then found = false
          if found then
            return true
    return false

chunk = (lst,chunk_size) ->
  c = (l,s,acc) ->
    if #l == 0 return acc
    t = {}
    for i = 1, s
      table.insert(t, l[i])
    table.insert(acc, t)
    return c([x for x in *l[s+1,]],s,acc)
  return c(lst,chunk_size,{})

parse_input = (input) ->
  i = [s for s in string.gmatch(input, "([^\n]*)")]
  choices = [ tonumber(item) for item in string.gmatch(i[1], "([^,]+)")]
  boards = [Board(table.concat(c, "\n")) for c in *chunk([item for item in *i[2,] when item ~= ""], 5)]
  return BingoGame(choices, boards)

partOne = (input) ->
  choice, winning_board = parse_input(input)\play!
  flat_board = {}
  for y in *winning_board.board
    for e in *y
      if (type(e) == "number") then table.insert(flat_board, e)
  return choice * fold(flat_board, (x,y) -> x + y)

partTwo = (input) ->
  game = parse_input(input)
  winning_board = nil
  choice = nil
  while #game.boards ~= 0
    choice, winning_board, i = game\play!
    table.remove(game.boards, i)
  flat_board = {}
  for y in *winning_board.board
    for e in *y
      if (type(e) == "number") then table.insert(flat_board, e)
  return choice * fold(flat_board, (x,y) -> x + y)

{
  :BingoGame,
  :Board,
  :parse_input,
  :partOne,
  :partTwo
}