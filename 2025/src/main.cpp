#include "aoc.hpp"
#include "dotenv.h"
#include <charconv>
#include <memory>
#include <print>
#include <span>
#include <string_view>

std::unique_ptr<aoc::Solution> get_solution(int day);

int main(int argc, char* argv[]) {
    std::span args(argv, argc);

    if (args.size() != 2) {
        std::println(stderr, "Usage: {} <day>", args[0]);
        return 1;
    }

    std::string_view day_str{args[1]};
    int day{};
    auto [ptr, ec] = std::from_chars(day_str.begin(), day_str.end(), day);
    if (ec != std::errc{} || day < 1 || day > 25) {
        std::println(stderr, "Error: day must be a number between 1 and 25");
        return 1;
    }

    auto solution = get_solution(day);
    if (!solution) {
        std::println(stderr, "Error: no solution found for day {}", day);
        return 1;
    }

    dotenv::init();
    auto token = std::getenv("SESSION");
    auto input = aoc::fetch_input(token, day);
    if (!input) {
        std::println(stderr, "Error: failed to fetch input for day {}", day);
        return 1;
    }

    std::println("Part 1: {}", solution->p1(*input));
    std::println("Part 2: {}", solution->p2(*input));

    return 0;
}
