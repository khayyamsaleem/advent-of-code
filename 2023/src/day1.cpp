#include <iostream>
#include <sstream>
#include <string>
#include <map>

namespace day1
{

    int get_two_digit_number_2(std::string s)
    {
        // start with a map of strings to ints, where the keys are
        // the english words for the numbers and the values are the numbers

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

        for (int i = 0; i < 9; i++) {
            numbers[std::to_string(i)] = i;
        }

        std::map<int, int> found_digits;
        for (auto const& [key, val] : numbers)
        {
            int pos = s.find(key);
            if (pos != std::string::npos) {
                std::cout << "found " << val << " at " << pos << " in " << s << std::endl;
                found_digits[val] = pos;
            }
        }

    }

    int get_two_digit_number(std::string s)
    {
        // iterate over s and first the first and last digit
        // concatenate them into a string and convert to int
        // return the int

        char first = '-';
        char second = '-';

        for (int i = 0; i < s.length(); i++)
        {
            if (isdigit(s[i]))
            {
                first = s[i];
                break;
            };
        }

        for (int i = s.length() - 1; i >= 0; i--)
        {
            if (isdigit(s[i]))
            {
                second = s[i];
                break;
            };
        }

        if (first == '-' || second == '-')
        {
            return 0;
        }

        std::string result = "";
        result += first;
        result += second;

        return std::stoi(result);
    }

    int solve_part1(std::string input)
    {
        int sum = 0;
        auto ss = std::stringstream{input};
        for (std::string line; std::getline(ss, line, '\n');)
        {
            sum += get_two_digit_number(line);
        }
        return sum;
    }

}
