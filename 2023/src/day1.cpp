#include <iostream>
#include <limits>
#include <sstream>
#include <string>
#include <map>
#include <vector>

namespace day1 {

    int get_two_digit_number_2(std::string s) {
        std::map<std::string, int> numbers = {
            {"zero", 0},
            {"one", 1},
            {"two", 2},
            {"three", 3},
            {"four", 4},
            {"five", 5},
            {"six", 6},
            {"seven", 7},
            {"eight", 8},
            {"nine", 9},
        };

        for (int i = 0; i < 10; i++) {
            numbers[std::to_string(i)] = i;
        }

        std::map<int, std::vector<int>> found_digits;
        for (auto const& [key, val] : numbers) {
            auto pos = s.find(key);
            while (pos != std::string::npos) {
                found_digits[val].push_back(pos);
                pos = s.find(key, pos+1);
            }
        }


        int minKey = -1, maxKey = -1;
        int minValue = std::numeric_limits<int>::max();
        int maxValue = std::numeric_limits<int>::min();

        for (const auto& [num, positions]: found_digits) {
            for (int pos: positions) {
                if (pos < minValue) {
                    minValue = pos;
                    minKey = num;
                }
                if (pos > maxValue) {
                    maxValue = pos;
                    maxKey = num;
                }
            }
        }

        return minKey * 10 + maxKey;

    }

    int get_two_digit_number(std::string s) {
        char first = '-';
        char second = '-';

        for (int i = 0; i < s.length(); i++) {
            if (isdigit(s[i])) {
                first = s[i];
                break;
            };
        }

        for (int i = s.length() - 1; i >= 0; i--) {
            if (isdigit(s[i])) {
                second = s[i];
                break;
            };
        }

        if (first == '-' || second == '-') {
            return 0;
        }

        std::string result = "";
        result += first;
        result += second;

        return std::stoi(result);
    }

    int solve_part1(std::string input) {
        int sum = 0;
        auto ss = std::stringstream{input};
        for (std::string line; std::getline(ss, line, '\n');) {
            sum += get_two_digit_number(line);
        }
        return sum;
    }

    int solve_part2(std::string input) {
        int sum = 0;
        auto ss = std::stringstream{input};
        for (std::string line; std::getline(ss, line, '\n');) {
            sum += get_two_digit_number_2(line);
        }
        return sum;
    }

}
