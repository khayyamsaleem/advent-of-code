#ifndef PUZZLE_REGISTRY_H
#define PUZZLE_REGISTRY_H

#include <functional>
#include <memory>
#include <unordered_map>

#include "puzzle.h"

namespace aoc {
using PuzzleRegistryFn = std::function<std::unique_ptr<Puzzle>()>;
using PuzzleRegistryMap = std::unordered_map<int, PuzzleRegistryFn>;

class PuzzleRegistry {
public:
  static PuzzleRegistryMap& getMap() {
    static PuzzleRegistryMap instance;
    return instance;
  }

  static void registerPuzzle(int day, PuzzleRegistryFn f) {
    getMap().emplace(day, f);
  }

  static std::unique_ptr<Puzzle> createPuzzle(int day) {
    auto it = getMap().find(day);
    if (it != getMap().end()) {
      return it->second();
    }
    return nullptr;
  }
};

} // namespace aoc

#define REGISTER_PUZZLE(day, className) \
  namespace { \
    struct Register { \
      Register() { \
        aoc::PuzzleRegistry::registerPuzzle(day, []() { return std::make_unique<className>(); }); \
      } \
    }; \
    static Register _register; \
  }

#endif // PUZZLE_REGISTRY_H
