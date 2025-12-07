#define CPPHTTPLIB_OPENSSL_SUPPORT
#include "aoc.hpp"
#include <httplib.h>

namespace aoc {

const std::string AOC_HOST = "https://adventofcode.com";
const std::string COOKIE_HEADER = "cookie";
const auto YEAR = 2025;

std::optional<std::string> fetch_input(std::string token, int day) {
    httplib::Client c(AOC_HOST);
    if (auto res = c.Get(std::format("/{}/day/{}/input", YEAR, day),
                         {{COOKIE_HEADER, std::format("session={}", token)}})) {
        return res->body;
    }
    return std::nullopt;
}
} // namespace aoc
