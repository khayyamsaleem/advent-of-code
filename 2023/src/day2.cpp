#include <iostream>
#include <map>
#include <numeric>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

#include <boost/algorithm/string.hpp>

#include "puzzles.h"

namespace aoc {

bool Day2::Game::operator==(const Game& other) const {
    if (id != other.id) return false;

    if (cubesets.size() != other.cubesets.size()) return false;

    for (size_t i = 0; i < cubesets.size(); ++i) {
        if (cubesets[i] != other.cubesets[i]) return false;
    }

    return true;
}

std::ostream& operator<<(std::ostream& os, const Day2::Game& game) {
    os << "Game " << game.id << ": " << std::endl;
    for (const auto& cubeset : game.cubesets) {
        os << "\t";
        for (const auto& [color, count]: cubeset) {
            os << color << " => " << count << ", ";
        }
        os << std::endl;
    }
    os << "\b\b\b\b";
    return os;
}

bool Day2::Game::is_possible_with_bag(std::map<std::string, int> bag) {
    for (const auto& draw : cubesets) {
        for (const auto& [color, count]: draw) {
            if (bag.at(color) < count) {
                return false;
            }
        }
    }
    return true;
}

int Day2::Game::get_power() {
    auto min_cubes = std::accumulate(
      cubesets.begin(),
      cubesets.end(),
      std::map<std::string, int>(),
      [](auto acc, const auto& draw) {
          for (const auto& [color, count]: draw) {
              if (acc.find(color) == acc.end() || count > acc[color]) {
                  acc[color] = count;
              }
          }
          return acc;
      }
    );

    return std::accumulate(
      min_cubes.begin(),
      min_cubes.end(),
      1,
      [](int power, const std::pair<const std::string, int> entry) {
          return power*entry.second;
      }
    );
}

Day2::Game Day2::parse_game(std::string line) {
    std::regex gamePattern("Game (\\d+): (.+)");
    std::regex cubesetsPattern("(\\d+) ([a-z]+)");

    std::smatch matches;
    Game g;
    if (std::regex_search(line, matches, gamePattern)) {
        g.id = std::stoi(matches[1]);

        auto ssx = std::stringstream{matches[2].str()};
        int i = 0;
        for (std::string cubeset; std::getline(ssx, cubeset, ';'); i++) {
            std::map<std::string, int> c;
            auto ssy = std::stringstream{cubeset};
            for (std::string cubeinfo; std::getline(ssy, cubeinfo, ',');) {
                std::smatch cmatches;
                if (std::regex_search(cubeinfo, cmatches, cubesetsPattern)) {
                    c[cmatches[2].str()] = std::stoi(cmatches[1].str());
                }
            }
            g.cubesets.push_back(c);
        }
    }
    return g;
}

int Day2::solve_part1(std::string input) {
    std::map<std::string, int> bag = {
        {"red", 12},
        {"green", 13},
        {"blue", 14}
    };

    std::vector<std::string> lines;
    boost::iter_split(lines, input, boost::first_finder("\n"));

    return std::accumulate(
      lines.begin(),
      lines.end(),
      0,
      [this, &bag](int total, const auto& line_iter) {
          if (line_iter.empty()) {
              return total;
          };
          Game g = parse_game(std::string(line_iter.begin(), line_iter.end()));
          return total + (g.is_possible_with_bag(bag) ? g.id : 0);
      }
    );
}

int Day2::solve_part2(std::string input) {
    std::vector<std::string> lines;
    boost::iter_split(lines, input, boost::first_finder("\n"));

    return std::accumulate(
      lines.begin(),
      lines.end(),
      0,
      [this](int total, const auto& line_iter) {
          if (line_iter.empty()) {
              return total;
          };
          Game g = parse_game(std::string(line_iter.begin(), line_iter.end()));
          return total + g.get_power();
      }
    );
}

void Day2::solve(std::string input) {
    std::cout << "2023 Day 2 Part 1: " << solve_part1(input) << std::endl;
    std::cout << "2023 Day 2 Part 2: " << solve_part2(input) << std::endl;
}

}
