#include <cstdio>

extern "C" {
    void solve(const char* input) {
        if (input == nullptr) {
            fprintf(stderr, "err: null input\n");
            return;
        }
        fprintf(stdout, "input:\n%s\n",input);
    }
}

