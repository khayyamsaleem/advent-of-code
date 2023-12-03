#include "day1.h"
#include <gtest/gtest.h>

TEST(Day1, Part1)
{
  EXPECT_EQ(day1::get_two_digit_number("1abc2"), 12);
  EXPECT_EQ(day1::get_two_digit_number("pqr3stu8vwx"), 38);
  EXPECT_EQ(day1::get_two_digit_number("a1b2c3d4e5f"), 15);
  EXPECT_EQ(day1::get_two_digit_number("treb7uchet"), 77);

  EXPECT_EQ(day1::solve_part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);

  EXPECT_EQ(day1::get_two_digit_number_2("two1nine"), 29);
  EXPECT_EQ(day1::get_two_digit_number_2("eighttwothree"), 83);
  EXPECT_EQ(day1::get_two_digit_number_2("abcone2threexyz"), 13);
  EXPECT_EQ(day1::get_two_digit_number_2("xtwone3four"), 24);

  EXPECT_EQ(day1::solve_part2(R"(two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
)"), 281);

}

int main(int argc, char **argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
