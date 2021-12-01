# AoC 2021 Day 01

## Prerequisites
Create a `.env` file:

```text
session=<advent-of-code-session-token>
```

## Running with Podman
```bash
podman build . -t hamthewhale/aoc-2021-day-01
podman run --rm hamthewhale/aoc-2021-day-01
```

## Running on macOS
```bash
brew install luarocks openssl@1.1
luarocks init
./luarocks install --tree lua_modules --only-deps aoc2021-dev-01.rockspec OPENSSL_DIR=/usr/local/opt/openssl@1.1/ CRYPTO_DIR=/usr/local/opt/openssl@1.1/ --local

./lua -l set_paths 01.run.lua
```

## Running Tests
```bash
./lua -l set_paths 01.test.lua
```