#include <iostream>
#include <sstream>
#include <string>

namespace day1
{
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
