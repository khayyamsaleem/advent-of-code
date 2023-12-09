#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <numeric>
#include <sstream>
#include <string>
#include <ranges>
#include <regex>

#include "puzzles.h"

namespace aoc {

Day4::Card::Card(std::string_view input) {
    std::regex regex(R"(Card\s+(\d+): (.+) \| (.+))");
    std::smatch match;

    auto nums = [&](const std::string &str) {
        std::set<int> numbers;
        std::regex numRegex(R"(\d+)");
        std::sregex_iterator it(str.begin(), str.end(), numRegex);
        std::sregex_iterator end;
        std::transform(it, end, std::inserter(numbers, numbers.begin()),
                        [](const std::smatch &m) { return std::stoi(m.str()); });
        return numbers;
    };

    std::string s(input);
    if (std::regex_search(s, match, regex) && match.size() > 3) {
        id = std::stoi(match[1].str());
        winningNumbers = nums(match[2].str());
        myNumbers = nums(match[3].str());
    } else {
        std::cerr << "input string: " << s << std::endl;
        throw std::invalid_argument("Invalid input string format for Card");
    }
}

int Day4::Card::wins() const {
    std::set<int> intersection;
    std::set_intersection(
        winningNumbers.begin(), winningNumbers.end(),
        myNumbers.begin(), myNumbers.end(),
        std::inserter(intersection, intersection.begin())
    );

    return intersection.size();
}

int Day4::Card::worth() const {
    auto i = wins();
    return i == 0 ? 0 : pow(2, i - 1);
}

std::vector<Day4::Card> parse_cards(const std::string& input) {
    std::istringstream iss(input);
    std::vector<std::string> lines;
    std::string line;

    while (std::getline(iss, line)) {
        if (!line.empty()) {
            lines.push_back(line);
        }
    }

    std::vector<Day4::Card> cards;
    for (const auto& l : lines) {
        cards.emplace_back(l);
    }

    return cards;
}

int Day4::solve_part1(std::string input) {
    auto cards = parse_cards(input);
    return std::accumulate(cards.begin(), cards.end(), 0, [](auto acc, const auto& c) { return acc + c.worth(); });
}

int Day4::solve_part2(std::string input) {
    auto cards = parse_cards(input);
    int total = 0;

    std::map<int, int> card_counts;

    for (auto& card : cards) {
        card_counts[card.id]++;
        for (int i = 0; i < card_counts[card.id]; ++i) {
            for (int j = 1; j <= card.wins(); ++j) {
                card_counts[card.id + j]++;
            }
        }
    }

    return std::accumulate(card_counts.begin(), card_counts.end(), 0, [](auto acc, auto cc) { return acc + cc.second; });
}

void Day4::solve(std::string input) {
    std::cout << "2023 Day 4 Part 1: " << solve_part1(input) << std::endl;
    std::cout << "2023 Day 4 Part 2: " << solve_part2(input) << std::endl;
}

}


