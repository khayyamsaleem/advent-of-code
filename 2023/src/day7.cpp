#include <iostream>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>
#include <ranges>
#include <fmt/ranges.h>

#include "puzzles.h"

namespace aoc {

const std::vector<char> CARDS = {'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'};

std::ostream& operator<<(std::ostream& os, const std::pair<std::string, int>& p) {
    return os << "(" << p.first << " -> " << p.second << ")";
}

std::ostream& operator<<(std::ostream& os, const std::vector<std::pair<std::string,int> >& vec) {
    os << "[";
    bool first = true;
    for (const auto& p : vec) {
        if (!first) os << ", " << std::endl;
        os << p;
        first = false;
    }
    return os << "]";
}

auto parse_input(const std::string& input) {
    auto lines = input | std::views::split('\n');

    std::vector<std::pair<std::string, int> > x;
    for (const auto& line : lines) {
      auto hand_and_bid = line | std::views::split(' ');
      auto it = hand_and_bid.begin();
      auto hand = fmt::format("{}",fmt::join(*it++, ""));
      int bid = std::stoi(fmt::format("{}",fmt::join(*it, "")));
      x.push_back(std::make_pair(hand, bid));
    }

    std::cout << x << std::endl;
}

int Day7::solve_part1(std::string input) {
  parse_input(input);
  return 0;
}

int Day7::solve_part2(std::string input) {
  return 0;
}

void Day7::solve(std::string input) {

}

}
