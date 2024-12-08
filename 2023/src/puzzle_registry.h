#ifndef PUZZLE_REGISTRY_H
#define PUZZLE_REGISTRY_H

#include <iostream>
#include <functional>
#include <memory>
#include <unordered_map>

#include "puzzle.h"
#include "puzzles.h"

namespace aoc {
using PuzzleRegistryFn = std::function<std::unique_ptr<Puzzle>()>;

class PuzzleRegistry {
public:
static std::unordered_map<int, PuzzleRegistryFn> make_registry() {
    return {
        {7, []() { return std::make_unique<Day7>(); }},
        {6, []() { return std::make_unique<Day6>(); }},
        {5, []() { return std::make_unique<Day5>(); }},
        {4, []() { return std::make_unique<Day4>(); }},
        {3, []() { return std::make_unique<Day3>(); }},
        {2, []() { return std::make_unique<Day2>(); }},
        {1, []() { return std::make_unique<Day1>(); }}
    };
}

};

} // namespace aoc

#endif // PUZZLE_REGISTRY_H
