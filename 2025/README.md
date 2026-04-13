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

