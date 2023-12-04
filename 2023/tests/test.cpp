#include <map>
#include <string>

#include <gtest/gtest.h>

#include "puzzle_registry.h"
#include "puzzles.h"

auto registry = aoc::PuzzleRegistry::make_registry();

TEST(Day1, Part1) {
  auto d1 = dynamic_cast<aoc::Day1*>(registry.at(1)().get());
  EXPECT_EQ(d1->get_two_digit_number("1abc2"), 12);
  EXPECT_EQ(d1->get_two_digit_number("pqr3stu8vwx"), 38);
  EXPECT_EQ(d1->get_two_digit_number("a1b2c3d4e5f"), 15);
  EXPECT_EQ(d1->get_two_digit_number("treb7uchet"), 77);

  EXPECT_EQ(d1->solve_part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
}

TEST(Day1, Part2) {
  auto d1 = dynamic_cast<aoc::Day1*>(registry.at(1)().get());
  EXPECT_EQ(d1->get_two_digit_number_2("two1nine"), 29);
  EXPECT_EQ(d1->get_two_digit_number_2("eighttwothree"), 83);
  EXPECT_EQ(d1->get_two_digit_number_2("abcone2threexyz"), 13);
  EXPECT_EQ(d1->get_two_digit_number_2("xtwone3four"), 24);

  EXPECT_EQ(d1->solve_part2(R"(two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
)"), 281);
}

TEST(Day2, Part1) {
  auto d2 = dynamic_cast<aoc::Day2*>(registry.at(2)().get());
  aoc::Day2::Game g = { 4, {
      {{"green", 1}, {"red", 3}, {"blue", 6}},
      {{"green", 3}, {"red", 6}},
      {{"green", 3}, {"blue", 15}, {"red", 14}}
  }};

  EXPECT_EQ(
    d2->parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
    g
  );

  std::map<std::string, int> bag = {
    {"red", 12},
    {"green", 13},
    {"blue", 14}
  };

  EXPECT_EQ(
    g.is_possible_with_bag(bag),
    false
  );

  EXPECT_EQ(
    d2
      ->parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
      .is_possible_with_bag(bag),
    true
  );

  auto test_input = R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
)";

  EXPECT_EQ(d2->solve_part1(test_input), 8);
}

TEST(Day2, Part2) {
  auto d2 = dynamic_cast<aoc::Day2*>(registry.at(2)().get());
  auto test_input = R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
)";

  EXPECT_EQ(d2->solve_part2(test_input), 2286);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
