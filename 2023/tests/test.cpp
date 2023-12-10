#include <map>
#include <string>

#include <gtest/gtest.h>

#include "puzzle_registry.h"
#include "puzzles.h"

auto registry = aoc::PuzzleRegistry::make_registry();

TEST(Day1, Part1) {
  auto registry_entry = registry.at(1)();
  auto d1 = dynamic_cast<aoc::Day1*>(registry_entry.get());
  EXPECT_EQ(d1->get_two_digit_number("1abc2"), 12);
  EXPECT_EQ(d1->get_two_digit_number("pqr3stu8vwx"), 38);
  EXPECT_EQ(d1->get_two_digit_number("a1b2c3d4e5f"), 15);
  EXPECT_EQ(d1->get_two_digit_number("treb7uchet"), 77);

  EXPECT_EQ(d1->solve_part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
}

TEST(Day1, Part2) {
  auto registry_entry = registry.at(1)();
  auto d1 = dynamic_cast<aoc::Day1*>(registry_entry.get());
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
  auto registry_entry = registry.at(2)();
  auto d2 = dynamic_cast<aoc::Day2*>(registry_entry.get());
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
  auto registry_entry = registry.at(2)();
  auto d2 = dynamic_cast<aoc::Day2*>(registry_entry.get());
  auto test_input = R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
)";

  EXPECT_EQ(d2->solve_part2(test_input), 2286);
}


TEST(Day3, Part1) {
  auto registry_entry = registry.at(3)();
  auto d3 = dynamic_cast<aoc::Day3*>(registry_entry.get());
  auto test_input = R"(467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..)";

  EXPECT_EQ(d3->solve_part1(test_input), 4361);
}

TEST(Day3, Part2) {
  auto registry_entry = registry.at(3)();
  auto d3 = dynamic_cast<aoc::Day3*>(registry_entry.get());
  auto test_input = R"(467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..)";
  EXPECT_EQ(d3->solve_part2(test_input), 467835);
}

TEST(Day4, Part1) {
  auto registry_entry = registry.at(4)();
  auto d4 = dynamic_cast<aoc::Day4*>(registry_entry.get());
  auto test_input = R"(Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11)";

  std::string single_card = "Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53";
  aoc::Day4::Card card(single_card);

  EXPECT_EQ(card.id, 1);
  EXPECT_EQ(card.winningNumbers.size(), 5);
  EXPECT_EQ(card.myNumbers.size(), 8);

  EXPECT_EQ(card.worth(), 8);

  EXPECT_EQ(d4->solve_part1(test_input), 13);
}

TEST(Day4, Part2) {
  auto registry_entry = registry.at(4)();
  auto d4 = dynamic_cast<aoc::Day4*>(registry_entry.get());
  auto test_input = R"(Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11)";
  EXPECT_EQ(d4->solve_part2(test_input), 30);
}

TEST(Day5, Part1) {
  auto registry_entry = registry.at(5)();
  auto d5 = dynamic_cast<aoc::Day5*>(registry_entry.get());
  auto test_input = R"(seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4)";
  EXPECT_EQ(d5->solve_part1(test_input), 35);
}

TEST(Day5, Part2) {
  auto registry_entry = registry.at(5)();
  auto d5 = dynamic_cast<aoc::Day5*>(registry_entry.get());
  auto test_input = R"(seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4)";
  EXPECT_EQ(d5->solve_part2(test_input), 46);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
