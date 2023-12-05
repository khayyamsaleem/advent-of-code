#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include <boost/algorithm/string.hpp>

#include "puzzles.h"

namespace aoc {

int is_symbol(char c) {
    return !isdigit(c) && c != '.';
}

bool is_adjacent_to_symbol(
    const std::pair<int, int>& num_start_pos,
    int number,
    const std::map<std::pair<int, int>, char>& symbols
) {
    int num_length = std::to_string(number).length();
    for (int i = 0; i < num_length; i++) {
        int x = num_start_pos.second + i;
        int y = num_start_pos.first;

        for (int dy = -1; dy <= 1; dy++) {
            for (int dx = -1; dx <= 1; dx++) {
                if (dy == 0 && dx == 0) continue;
                if (symbols.find(std::make_pair(y + dy, x + dx)) != symbols.end()) return true;
            }
        }
    }
    return false;
}

void find_symbols_and_numbers(
    std::string input,
    std::map<std::pair<int,int>,char>& symbols,
    std::map<std::pair<int,int>,int>& numbers
) {
    std::vector<std::string> lines;
    boost::iter_split(lines, input, boost::first_finder("\n"));

    for (auto it = lines.begin(); it != lines.end(); ++it) {
        const std::string& line = *it;
        int y = std::distance(lines.begin(), it);
        for (int x = 0; x < line.length(); x++) {
            char c = line[x];
            if (is_symbol(c)) symbols.emplace(std::make_pair(y, x), c);
            if (isdigit(c)) {
                int start = x;
                int n;
                for (n = 0; x < line.length() && isdigit(line[x]); n = n * 10 + (line[x] - '0'), x++);
                x--;
                numbers.emplace(std::make_pair(y, start), n);
            }
        }
    }
}

int Day3::solve_part1(std::string input) {
    std::map<std::pair<int,int>, char> symbols;
    std::map<std::pair<int,int>,int> numbers;
    find_symbols_and_numbers(input, symbols, numbers);

    return std::accumulate(
        numbers.begin(),
        numbers.end(),
        0,
        [&symbols](int acc, const std::pair<std::pair<int,int>,int>& entry) {
            const auto& [coord, num] = entry;
            return acc + (is_adjacent_to_symbol(coord, num, symbols) ? num : 0);
        }
    );
}

int Day3::solve_part2(std::string input) {
    std::vector<std::string> lines;
    boost::iter_split(lines, input, boost::first_finder("\n"));

    int total_gear_ratio = 0;

    for (int y = 0; y < lines.size(); ++y) {
        for (int x = 0; x < lines[y].size(); ++x) {
            if (lines[y][x] == '*') {
                std::vector<std::pair<int, std::pair<int, int>>> parts;

                for (int dy = -1; dy <= 1; dy++) {
                    for (int dx = -1; dx <= 1; dx++) {
                        if (dy == 0 && dx == 0) continue;

                        int checkY = y + dy;
                        int checkX = x + dx;

                        if (checkY >= 0 && checkY < lines.size() && checkX >= 0 && checkX < lines[checkY].size() && isdigit(lines[checkY][checkX])) {
                            int startX = checkX;
                            while (startX > 0 && isdigit(lines[checkY][startX - 1])) {
                                startX--;
                            }

                            std::pair<int, int> pos = {checkY, startX};
                            if (std::find_if(
                                  parts.begin(),
                                  parts.end(),
                                  [pos](const std::pair<int, std::pair<int, int>>& elem) {return elem.second == pos;}
                                ) == parts.end()
                            ) {
                                int number = 0;
                                for (int i = startX; i < lines[checkY].size() && isdigit(lines[checkY][i]); i++) {
                                    number = number * 10 + (lines[checkY][i] - '0');
                                }
                                parts.emplace_back(number, pos);
                            }
                        }
                    }
                }

                if (parts.size() == 2) {
                    total_gear_ratio += parts[0].first * parts[1].first;
                }
            }
        }
    }

    return total_gear_ratio;
}


void Day3::solve(std::string input) {
    std::cout << "2023 Day 3 Part 1: " << solve_part1(input) << std::endl;
    std::cout << "2023 Day 3 Part 2: " << solve_part2(input) << std::endl;
}

}


