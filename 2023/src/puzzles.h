#include "puzzle.h"

#include <string>

namespace aoc {
class Day1 : public Puzzle {
public:
  int get_two_digit_number(std::string input);
  int get_two_digit_number_2(std::string input);
  int solve_part1(std::string input);
  int solve_part2(std::string input);
  void solve(std::string input);
};
class Day2 : public Puzzle {
public:
  void solve(std::string input);
  int solve_part1(std::string input);
};
}
