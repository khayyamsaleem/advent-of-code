#ifndef PUZZLE_H
#define PUZZLE_H

#include <string>

namespace aoc {

class Puzzle {
public:
    virtual void solve(std::string input) = 0;
    virtual ~Puzzle() {}
};

} // namespace aoc

#endif // PUZZLE_H
