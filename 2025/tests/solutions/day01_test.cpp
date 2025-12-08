#include <catch2/catch_test_macros.hpp>
#include "solutions/day01.hpp"

TEST_CASE("Day 01 helpers", "[day01]") {

    auto test_input = R"(
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    )";

    SECTION("Parse input") {
        auto result = day01::parse(test_input);
        REQUIRE(std::ranges::distance(result) == 10);
    }

    SECTION("Spin") {
        auto d = day01::spin(day01::parse(test_input));
        REQUIRE(d.zero_count == 3);
    };
}
