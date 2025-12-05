#include <catch2/catch_test_macros.hpp>
#include "aoc.hpp"

TEST_CASE("Basic Math Test", "[math]") {
    int a = 2;
    int b = 3;
    REQUIRE( a + b == 5 );
}
