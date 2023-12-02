#include <iostream>
#include <dotenv.h>
#include <libaoc/get_input.h>

#include "day1.h"

int main()
{
  auto &dotenv = dotenv::env.load_dotenv();

  std::cout << "2023 Day 1 Part 1: " << day1::solve_part1(get_input(dotenv["SESSION"], 2023, 1)) << std::endl;
  return 0;
}