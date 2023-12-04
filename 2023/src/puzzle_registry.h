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
        {1, []() { return std::make_unique<Day1>(); }},
        {2, []() { return std::make_unique<Day2>(); }}
    };
}

};

} // namespace aoc

#endif // PUZZLE_REGISTRY_H
