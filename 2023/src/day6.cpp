#include <iostream>
#include <iterator>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>

#include "puzzles.h"


namespace aoc {

constexpr auto TIME_PREFIX = sizeof("Time:") - 1;
constexpr auto DIST_PREFIX = sizeof("Distance:") - 1;

void Day6::solve(std::string input) {
    std::istringstream ss(input);

    std::vector<int> times, distances;

    auto parse_ints = [](const std::string& input, int prefix_size) {
        std::istringstream int_stream(input.substr(prefix_size));
        return std::vector<int>{std::istream_iterator<int>(int_stream), std::istream_iterator<int>()};
    };

    for (std::string line; std::getline(ss,line);) {
        std::istringstream lss(line);
        std::string prefix;
        std::getline(lss, prefix, ' ');
        switch(prefix.length()) {
            case TIME_PREFIX:
                times = parse_ints(line, TIME_PREFIX);
                break;
            case DIST_PREFIX:
                distances = parse_ints(line, DIST_PREFIX);
                break;
            default:
                throw std::invalid_argument("bad");
        };
    }

    auto res = 1;
    for (
        auto itT = times.begin(), itD = distances.begin();
        itT != times.end() && itD != distances.end();
        ++itT, ++itD
    ) {
        auto wins = 0;
        for (auto i = 0; i < *itT; i++) wins += (i*(*itT - i) > *itD) ? 1 : 0;
        res *= wins;
    }

    std::cout << "Day 6 Part 1: " << res << std::endl;

    long long time = std::stoll(std::accumulate(times.begin(), times.end(), std::string(), [](auto acc, auto num) {
        return acc + std::to_string(num);
    }));

    long long dist = std::stoll(std::accumulate(distances.begin(), distances.end(), std::string(), [](auto acc, auto num) {
        return acc + std::to_string(num);
    }));

    auto wins = 0;
    for (auto i = 0; i < time; i++) wins += (i*(time - i) > dist) ? 1 : 0;

    std::cout << "Day 6 Part 2: " << wins << std::endl;

}

}
