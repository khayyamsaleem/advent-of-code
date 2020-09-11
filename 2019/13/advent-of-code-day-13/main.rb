require 'json'
require 'dotenv/load'
require_relative './lib/intcode_client'
require_relative './lib/aoc_client'

TILES = {
  0 => ' ',
  1 => '|',
  2 => '#',
  3 => '-',
  4 => 'O'
}.freeze

def create_grid(width, length)
  grid = []
  (0..length).each do |_i|
    x = []
    (0..width).each do |_j|
      x << :e
    end
    grid << x
  end
  grid
end

def populate_grid(grid, program_output)
  program_output.each_slice(3) do |s|
    if s[0..1] != [-1, 0]
        grid[s[1]][s[0]] = TILES[s[2]]
    end
  end
  grid
end

def display_grid(grid)
  grid.each do |row|
    puts row.join('')
  end
  puts ((0..grid[0].length).to_a.map { |x| x % 10 }).join('')
end

def insert_two_quarters(puzzle_input)
  head, *tail = puzzle_input
  return [2] + tail
end

def part_one(intcode_client, puzzle_input)
  program_output = (intcode_client.eval_intcode puzzle_input)['output-signals']
  num_blocks = 0
  program_output.each_slice(3) do |s|
    num_blocks += 1 if s[2] == 2
  end
  num_blocks
end

def part_two(intcode_client, puzzle_input)
  program_counter = 0
  inputs = []
  output_signals = []
  score = 0
  grid = create_grid(36, 19)
  ball_pos_x = -1
  paddle_pos_x = -1
  program = puzzle_input
  loop do
    program_output = intcode_client.eval_intcode(insert_two_quarters(program), inputs, program_counter)
    program_counter = program_output['program-counter']
    output_signals = program_output['output-signals']
    program = program_output['program']
    grid = populate_grid(grid, output_signals)
    display_grid(grid)
    output_signals.each_slice(3) do |slice|
        case slice
        in [-1, 0, _score]
            score = _score
        in [x, y, 4] if x > 0
            ball_pos_x = x
        in [x, y, 3] if x > 0
            paddle_pos_x = x
        else
        end
    end
    puts "Score: #{score}"
    inputs = [ paddle_pos_x > ball_pos_x ? -1 : paddle_pos_x < ball_pos_x ? 1 : 0]
    p inputs
    p "paddle: #{paddle_pos_x}"
    p "ball: #{ball_pos_x}"
    break if !grid.flatten.include? '#'
  end 
  return score
end

def main
  intcode_client = IntcodeClient.new ENV['intcode_server_uri']
  aoc_client = AOCClient.new('https://adventofcode.com', ENV['session'])

  puzzle_input = JSON.parse "[#{aoc_client.get_input_for_year_and_day(2019, 13)}]"

  part_one_solution = part_one(intcode_client, puzzle_input)
  puts "Part One: #{part_one_solution}"

  part_two(intcode_client, puzzle_input)
end

main
