#include <catch2/catch_test_macros.hpp>
#include "solutions/day01.hpp"

TEST_CASE("Day 01 helpers", "[day01]") {

    SECTION("Parse input") {
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
        auto result = day01::parse(test_input);
        REQUIRE(std::ranges::distance(result) == 10);
    }

    SECTION("Spin p1") {
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
        auto d = day01::spin(day01::parse(test_input), day01::d1p1);
        REQUIRE(d.zero_count == 3);
    };

    SECTION("Spin p2") {
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
        auto d = day01::spin(day01::parse(test_input), day01::d1p2);
        REQUIRE(d.zero_count == 6);
    };

    SECTION("Spin p2 - multiple rounds") {
        auto test_input = R"(
        R1000
        )";
        auto d = day01::spin(day01::parse(test_input), day01::d1p2);
        REQUIRE(d.zero_count == 10);
    };

}
