# AoC 2021 -- Lua

![Lua logo](https://upload.wikimedia.org/wikipedia/commons/c/cf/Lua-Logo.svg | width=100)

## Prerequisites
Create a `.env` file:

```text
session=<advent-of-code-session-token>
```

## Running with Podman
```bash
podman build . -t hamthewhale/aoc-2021-lua
podman run --rm hamthewhale/aoc-2021-lua
```

## Running on macOS
```bash
brew install luarocks openssl@1.1
luarocks init
./luarocks install --tree lua_modules --only-deps aoc2021-dev-01.rockspec OPENSSL_DIR=/usr/local/opt/openssl@1.1/ CRYPTO_DIR=/usr/local/opt/openssl@1.1/ --local

./lua run.lua <day>
```

## Running Tests
```bash
./lua -l set_paths 01/test.lua
./lua -l set_paths 02/test.lua
```