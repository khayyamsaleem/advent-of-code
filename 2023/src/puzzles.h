#ifndef PUZZLES_H
#define PUZZLES_H

#include <string>
#include <map>
#include <vector>
#include <set>
#include "puzzle.h"

namespace aoc {

class Day1 : public Puzzle {
public:
  int get_two_digit_number(std::string input);
  int get_two_digit_number_2(std::string input);
  int solve_part1(std::string input);
  int solve_part2(std::string input);
  void solve(std::string input) override;
};

class Day2 : public Puzzle {
public:
  struct Game {
    int id;
    std::vector<std::map<std::string,int> > cubesets;

    friend std::ostream& operator<<(std::ostream& os, const Game& game);
    bool operator==(const Game& other) const;
    bool is_possible_with_bag(std::map<std::string,int> bag);
    int get_power();
  };
  void solve(std::string input) override;
  int solve_part1(std::string input);
  int solve_part2(std::string input);
  Game parse_game(std::string input);
};

class Day3 : public Puzzle {
public:
  void solve(std::string input) override;
  int solve_part1(std::string input);
  int solve_part2(std::string input);
};

class Day4 : public Puzzle {
public:
  struct Card {
    int id;
    std::set<int> winningNumbers, myNumbers;

    explicit Card(const std::string_view input);
    int worth() const;
    int wins() const;
  };
  void solve(std::string input) override;
  int solve_part1(std::string input);
  int solve_part2(std::string input);
};

}

#endif // PUZZLES_H
