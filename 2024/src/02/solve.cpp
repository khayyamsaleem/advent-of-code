#include <cstdio>
#include <cmath>
#include <sstream>
#include <string>
#include <iterator>
#include <vector>

bool isSafe(std::string line) {
    std::istringstream ss(line);
    int trend = 0;
    int last;
    ss >> last;
    int level;
    while (ss >> level) {
        int diff = std::abs(level - last);
        if (diff < 1 || diff > 3) return false;
        if (level > last) {
            if (trend < 0) return false;
            trend++;
        } else if (level < last) {
            if (trend > 0) return false;
            trend--;
        }
        last = level;
    }
    return true;
}

// this is super dumb but I can't be bothered rn lmao
// ideally I'd just count the violations and update last -> last-1
bool isSafeWithProblemDampener(std::string line) {
    if (isSafe(line)) return true;

    std::istringstream ss(line);
    std::vector<int> levels{std::istream_iterator<int>(ss), std::istream_iterator<int>()};

    for (size_t i = 0; i < levels.size(); ++i) {
        std::vector<int> perm = levels;
        perm.erase(perm.begin() + i);
        std::ostringstream oss;
        for (size_t j = 0; j < perm.size(); ++j) {
            if (j > 0) oss << " ";
            oss << perm[j];
        }
        if (isSafe(oss.str())) return true;
    }

    return false;
}

void solveInner(const char* input) {
    std::istringstream s(input);
    std::string line;
    int numSafe = 0;

    while (std::getline(s, line)) {
        numSafe += isSafe(line);
    }
    fprintf(stdout, "Day 02 - Part 1: %d\n", numSafe);

    std::istringstream ss(input);
    int numSafeWithProblemDampener = 0;
    while (std::getline(ss, line)) {
        numSafeWithProblemDampener += isSafeWithProblemDampener(line);
    }
    fprintf(stdout, "Day 02 - Part 2: %d\n", numSafeWithProblemDampener);
}

extern "C" {
    void solve(const char* input) {
        if (input == nullptr) {
            fprintf(stderr, "err: null input\n");
            return;
        }
        solveInner(input);
    }
}


//test
int main() {
    const char* test = R"(7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9)";
    solveInner(test);
}
