#include <iostream>
#include <string>

#include "puzzle_registry.h"

namespace aoc {
class Day2 : public Puzzle {
public:
    int solve_part1(std::string input) {
        return 0;
    }

    int solve_part2(std::string input) {
        return 0;
    }

    void solve(std::string input) override {
        std::cout << "2023 Day 2 Part 1: " << solve_part1(input) << std::endl;
        std::cout << "2023 Day 2 Part 2: " << solve_part2(input) << std::endl;
    }
};

REGISTER_PUZZLE(2, Day2)

}
