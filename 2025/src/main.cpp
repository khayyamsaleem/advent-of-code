#include "aoc.hpp"
#include <print>

int main() {
    auto token = "";
    std::println("{}", aoc::fetch_input(token, 1).value_or(""));
    return 0;
}
