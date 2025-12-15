#include <string>
#include <string_view>
#include "solutions/day01.hpp"

std::string Day01::p1(std::string_view input) {
    return std::to_string(day01::spin(day01::parse(input), day01::d1p1).zero_count);
}

std::string Day01::p2(std::string_view input) {
    return std::to_string(day01::spin(day01::parse(input), day01::d1p2).zero_count);
}
