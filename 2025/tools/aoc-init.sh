#!/bin/bash

set -e

DAY=$1
DAY_PADDED=$(printf "%02d" $1)

HPP_FILE="$BUILD_WORKSPACE_DIRECTORY/include/solutions/day${DAY_PADDED}.hpp"
CPP_FILE="$BUILD_WORKSPACE_DIRECTORY/src/solutions/day${DAY_PADDED}.cpp"
TEST_FILE="$BUILD_WORKSPACE_DIRECTORY/tests/solutions/day${DAY_PADDED}_test.cpp"

mkdir -p "$(dirname "$HPP_FILE")"
mkdir -p "$(dirname "$CPP_FILE")"
mkdir -p "$(dirname "$TEST_FILE")"

if [ -f "$HPP_FILE" ] || [ -f "$CPP_FILE" ] || [ -f "$TEST_FILE" ]; then
    echo "Error: Files for day ${DAY} already exist."
    exit 1
fi

cat > $HPP_FILE << EOL
#include "aoc.hpp"
#include <string_view>

namespace day${DAY_PADDED} {

inline auto parse(std::string_view input) {
    return input;
}

} // namespace day${DAY_PADDED}

class Day${DAY_PADDED} : public aoc::Solution {
  public:
    virtual std::string p1(std::string_view input) override;
    virtual std::string p2(std::string_view input) override;
};
EOL

cat > $CPP_FILE << EOL
#include "solutions/day${DAY_PADDED}.hpp"
#include <string>

std::string Day${DAY_PADDED}::p1(std::string_view input) {
    auto parsed = day${DAY_PADDED}::parse(input);
    (void)parsed;
    return "p1";
}

std::string Day${DAY_PADDED}::p2(std::string_view input) {
    auto parsed = day${DAY_PADDED}::parse(input);
    (void)parsed;
    return "p2";
}
EOL

cat > $TEST_FILE << EOL
#include <catch2/catch_test_macros.hpp>
#include "solutions/day${DAY_PADDED}.hpp"

TEST_CASE("Day ${DAY_PADDED} helpers", "[day${DAY_PADDED}]") {
    SECTION("Parse") {
        auto test_input = R"()";
        auto result = day${DAY_PADDED}::parse(test_input);
        (void)result;
        REQUIRE(true);
    }
}
EOL

echo "Created \${HPP_FILE}, \${CPP_FILE}, and \${TEST_FILE}"
