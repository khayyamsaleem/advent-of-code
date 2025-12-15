#include "aoc.hpp"
#include <algorithm>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <ostream>
#include <ranges>
#include <string_view>

namespace day01 {

  using PASSWORD_METHOD = int;

  const PASSWORD_METHOD d1p1 = 0x0;
  const PASSWORD_METHOD d1p2 = 0x434C49434B;
  const int DIAL_SIZE = 100;
  const int DIAL_START = 50;

  class rotation {
    public:
      rotation(int dir, int clicks) : dir(dir), clicks(clicks) {}
      int dir, clicks;

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
          return rotation{sv[0] == 'L' ? -1 : 1, std::stoi(std::string(sv.substr(1)))};
        });
  }

  struct dial {
    int val, zero_count;
  };

  inline auto spin(auto rotations, const PASSWORD_METHOD pm) {
    return std::ranges::fold_left(rotations, dial{DIAL_START, 0}, [pm](auto acc, rotation r) {
        if (pm == d1p2) {
            if (r.dir > 0) { 
                acc.zero_count += (acc.val + r.clicks) / DIAL_SIZE;
            } else { 
                auto dist = (acc.val == 0) ? DIAL_SIZE : acc.val;
                acc.zero_count += (r.clicks + DIAL_SIZE - dist) / DIAL_SIZE;
            }
        }

        auto move = r.dir * (r.clicks % DIAL_SIZE);
        acc.val = (acc.val + move + DIAL_SIZE) % DIAL_SIZE;
        if (pm == d1p1 && acc.val == 0) {
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
