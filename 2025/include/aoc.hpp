#pragma once
#include <string>
#include <optional>

namespace aoc {
std::optional<std::string> fetch_input(std::string token, int day);

class Solution {
public:
    virtual ~Solution() = default;

    virtual std::string p1(std::string_view input) = 0;
    virtual std::string p2(std::string_view input) = 0;
};

} // namespace aoc
