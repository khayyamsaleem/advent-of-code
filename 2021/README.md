# AoC 2021 -- Lua

<img src="https://upload.wikimedia.org/wikipedia/commons/c/cf/Lua-Logo.svg" width=100 />

## Prerequisites
Create a `.env` file:

```text
session=<advent-of-code-session-token>
```

## Running with Podman
```bash
podman build . -t hamthewhale/aoc-2021-lua
podman run --rm -it hamthewhale/aoc-2021-lua sh
```

## Running on macOS
```bash
brew install luarocks openssl@1.1
luarocks init
./luarocks install --tree lua_modules --only-deps aoc2021-scm-0.rockspec OPENSSL_DIR=/usr/local/opt/openssl@1.1/ CRYPTO_DIR=/usr/local/opt/openssl@1.1/ --local

./lua run.lua <day>
```

## Running Tests
```bash
./lua test.lua <day>
```
