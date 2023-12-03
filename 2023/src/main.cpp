#include <iostream>
#include <string>

#include <dotenv.h>
#include <libaoc/get_input.h>
#include "puzzle_registry.h"

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <day number or \"all\"" << std::endl;
    return 1;
  }

  std::string day = argv[1];

  auto &dotenv = dotenv::env.load_dotenv();

  if (day == "all") {
    for (const auto& [day, puzzle]: aoc::PuzzleRegistry::getMap()) {
      std::string input = get_input(dotenv["SESSION"], 2023, day);
      puzzle()->solve(input);
    }
    return 0;
  };

  int dayNum = std::stoi(day);
  auto puzzle = aoc::PuzzleRegistry::createPuzzle(dayNum);
  if (puzzle) {
    puzzle->solve(get_input(dotenv["SESSION"], 2023, dayNum));
  } else {
    std::cerr << "Puzzle for day " << dayNum << " not implemented." << std::endl;
    return 1;
  }

  return 0;
}
