#include "aoc.hpp"
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <ostream>
#include <ranges>
#include <string_view>

namespace day01 {
    class rotation {
        public:
            rotation(char dir, int clicks) : dir(dir), clicks(clicks) {}
            char dir;
            int clicks;

            friend std::ostream& operator<<(std::ostream& os, const rotation& r) {
                return os << '[' << r.dir << " | " << std::setw(4) << r.clicks << ']';
            }
    };

    inline auto parse(std::string_view input) {
        return input
            | std::views::split('\n')
            | std::views::filter([](auto&& line) {
                return std::ranges::any_of(line, [](auto c) {
                    return !std::isspace(c);
                });
            })
            | std::views::transform([](auto&& line) {
                auto sv = std::string_view(line.begin(), line.end());
                auto start = sv.find_first_not_of(" \t\r\n");
                if (start != std::string_view::npos) {
                    sv = sv.substr(start);
                }
                return rotation{sv[0], std::stoi(std::string(sv.substr(1)))};
            });
    }

    struct dial{int val, zero_count;};

    inline auto spin(auto rotations) {
        return std::ranges::fold_left(rotations, dial{50, 0}, [](auto acc, rotation r) {
            switch (r.dir) {
                case 'L':
                    acc.val = acc.val - r.clicks;
                    break;
                case 'R':
                    acc.val = acc.val + r.clicks;
                    break;
            }
            acc.val = ((acc.val % 100) + 100) % 100;

            if (acc.val == 0) {
                acc.zero_count++;
            }
            return acc;
        });
    }

} // namespace day01

class Day01 : public aoc::Solution {
public:
    std::string p1(std::string_view input) override;
    std::string p2(std::string_view input) override;
};
