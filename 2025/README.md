# aoc 2025

this year, back to c++ but building with bazel!

## build

```bash
bazel build //:aoc2025
```

## generate new solution starter code

```bash
bazel run //:aoc-init -- $DAY
```

## solving a day in go

drop two files in `go/dayXX/`, nothing else:

- `go/dayXX/dayXX.go` — `package dayXX`, exporting `P1(string) string` and `P2(string) string`
- `go/dayXX/dayXX_test.go` — standard go tests

bazel discovers them by glob. `tools/go_solutions.bzl` declares a `go_library` +
`go_test` per day and compiles all go days into one c-archive (each archive
embeds its own go runtime, so there can only be one). `tools/dispatcher.bzl`
generates the `aoc::Solution` subclass that marshals across the cgo boundary and
the `get_solution` case, so go days run under `bazel run //:aoc2025 -- $DAY` and
are tested by `bazel test //...` exactly like c++ days.

input fetching stays in c++ — `main.cpp` fetches once and hands the string to
whichever solution the dispatcher picks, regardless of implementation language.

## test

```bash
bazel test //... --test_output=all # run all tests
bazel coverage //... # produce coverage
tools/coverage.sh # coverage + HTML report (requires lcov)
```

## run

```bash
bazel run //:aoc2025
```

## generate `compile_commands.json` for ide support

```bash
bazel run //:refresh_compile_commands
```

