#include <iostream>
#include <string>
#include <thread>
#include <vector>

#include <dotenv.h>
#include <libaoc/get_input.h>

#include "puzzle_registry.h"
#include "puzzles.h"

const auto SESSION = "SESSION";

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <day number or \"all\"" << std::endl;
    return 1;
  }

  std::string day = argv[1];

  auto registry = aoc::PuzzleRegistry::make_registry();
  auto &dotenv = dotenv::env.load_dotenv();


  if (day == "all") {
    std::vector<std::thread> threads;
    for (const auto& [day_num, puzzle]: registry) {
      std::string input = get_input(dotenv[SESSION], 2023, day_num);
      threads.emplace_back([puzzle,input]() {
        puzzle()->solve(input);
      });
    }
    for (auto& t : threads) {
        t.join();
    }
    return 0;
  };

  int day_num = std::stoi(day);
  auto puzzle = registry.at(day_num)();
  puzzle->solve(get_input(dotenv[SESSION], 2023, day_num));

  return 0;
}
