# Advent of Code 2023

In which I find out just how far behind the dev experience is for C++ in 2023.

## Setup

Install [vcpkg](https://vcpkg.io/en/getting-started) and CMake.


```bash
git clone https://github.com/Microsoft/vcpkg.git $HOME/.vcpkg
$HOME/.vcpkg/bootstrap-vcpkg.sh
```

Configure the vcpkg-managed dependencies:

```bash
$HOME/.vcpkg/vcpkg install
```

## Build 

```bash
cmake -B build -S . \
   -DCMAKE_TOOLCHAIN_FILE=$HOME/.vcpkg/scripts/buildsystems/vcpkg.cmake \ -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
cmake --build build
```

## Run

Create a `.env` file in the root of the repo with contents:

```text
SESSION=<advent of code session token>
```

```bash
./build/src/aoc2023
```

## Test

```bash
./build/tests/test_runner
```
