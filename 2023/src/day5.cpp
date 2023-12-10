#include <climits>
#include <cstring>
#include <sstream>
#include <iostream>
#include <vector>
#include <map>
#include <cstring>
#include <string>
#include <regex>
#include <thread>
#include <algorithm>
#include <iterator>
#include <numeric>
#include <array>

#include "puzzles.h"

namespace aoc {

using transformer = std::vector<std::array<long long,3>>;
using transformer_type = std::pair<std::string, transformer>;
using transformers = std::map<std::string,transformer_type>;

std::ostream& operator<<(std::ostream& os, const transformer& tf) {
    for (const auto& rule : tf) os << "  [" << rule[0] << ", " << rule[1] << ", " << rule[2] << "]" << std::endl;
    return os;
}

std::ostream& operator<<(std::ostream& os, const std::map<std::string, transformer_type>& transformers) {
    for (const auto& pair : transformers) {
        const auto& source = pair.first;
        const auto& [destination, rules] = pair.second;

        os << source << "-to-" << destination << " map:" << std::endl;
        os << rules;
    }
    return os;
}

transformer mk_transformer(std::istringstream& ss) {
    transformer rules;
    std::string line;
    while (std::getline(ss, line) && !line.empty()) {
        std::istringstream lss(line);
        std::array<long long, 3> rule;
        auto i = 0;
        for (std::string c; std::getline(lss, c, ' '); i++) {
            rule[i] = std::stoll(c);
        }
        rules.push_back(rule);
    }
    return rules;
}

std::pair<std::vector<long long>, std::map<std::string, transformer_type>> process_input(const std::string& input) {
    const auto seeds_prefix = "seeds: ";
    std::istringstream ss(input);
    std::vector<long long> seeds;
    transformers transformers;

    const std::regex map_title_regex("([a-z]+)-to-([a-z]+) map:");
    std::string line;

    auto parse_seeds = [&seeds_prefix](const std::string& line) {
        std::istringstream seed_stream(line.substr(std::strlen(seeds_prefix)));
        return std::vector<long long>{std::istream_iterator<long long>(seed_stream), std::istream_iterator<long long>()};
    };

    while (std::getline(ss, line)) {
        if (line.substr(0, std::strlen(seeds_prefix)).compare(seeds_prefix) == 0) {
            seeds = parse_seeds(line);
        } else {
            std::smatch match;
            if (std::regex_search(line, match, map_title_regex) && match.size() == 3) {
                transformers[match[1]] = std::make_pair(match[2], mk_transformer(ss));
            }
        }
    }

    return std::make_pair(seeds, transformers);
}

long long apply_transformers(long long input, const std::string& current_type, const transformers transformers) {
    auto it = transformers.find(current_type);
    if (it == transformers.end()) return input;

    const auto& [destination_type, rules] = it->second;

    for (const auto& rule : rules) {
        const auto dst = rule[0];
        const auto src = rule[1];
        const auto length = rule[2];

        if (input >= src && input < src + length) {
            long long new_input = dst + (input - src);
            return apply_transformers(new_input, destination_type, transformers);
        }
    }

    return apply_transformers(input, destination_type, transformers);
}

long long Day5::solve_part1(std::string input) {
    auto [seeds, transformers] = process_input(input);

    return std::accumulate(seeds.begin(), seeds.end(), LLONG_MAX, [&transformers](long long current_min, long long seed) {
        return std::min(current_min, apply_transformers(seed, "seed", transformers));
    });

}

long long Day5::solve_part2(std::string input) {

    auto [seeds, transformers] = process_input(input);

    std::vector<std::pair<long long, long long>> seed_ranges;
    for (size_t i = 0; i < seeds.size(); i += 2) {
        seed_ranges.emplace_back(seeds[i], seeds[i + 1]);
    }

    std::vector<std::thread> threads;
    std::vector<long long> local_mins(seed_ranges.size(), LLONG_MAX);

    size_t i = 0;
    for (const auto& range : seed_ranges) {
        threads.emplace_back([this, &transformers, &local_mins, i, range]() {
            std::cout << "started range [" << range.first << ", " << range.second + range.first << ")" << std::endl;
            for (long long seed = range.first; seed < range.first + range.second; ++seed) {
                long long transformed_seed = apply_transformers(seed, "seed", transformers);
                local_mins[i] = std::min(local_mins[i], transformed_seed);
            }
            std::cout << "finished range [" << range.first << ", " << range.second + range.first << ")" << std::endl;
        });
        ++i;
    }

    for (auto& t : threads) {
        t.join();
    }

    long long min_result = *std::min_element(local_mins.begin(), local_mins.end());
    return min_result;

}

void Day5::solve(std::string input) {
    std::cout << "2023 Day 5 Part 1: " << solve_part1(input) << std::endl;
    std::cout << "2023 Day 5 Part 2: " << solve_part2(input) << std::endl;
}

}

