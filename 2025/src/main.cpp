#include "aoc.hpp"
#include <iostream>

int main() {
    auto token = "";
    std::cout << aoc::fetch_input(token, 1).value_or("") << std::endl;
    return 0;
}
