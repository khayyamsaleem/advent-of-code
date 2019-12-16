## AOC 2019 Day 7

> **Note**: This one was particularly wily, especially with my "new language per-day" insistence and the fact that I implemented my last intcode interpreter in racket. I extended my day 5 solution to include an intcode-evaluator-as-a-service. Because the racket web-servlet is not very performant, I found that running around 15-20 replicas of the servlet gave me something close to tolerable.

## Prereqs

### Environment
Create `.env` file with:

```
session=<aoc session cookie from browser>
intcode_server=<uri for intcode server> # http://localhost:1337 or http://intcode.docker.localhost
```

### Intcode Server
Run the intcode server from day 5 with docker, by running `docker-compose up -d --scale intcode-server=15`

If you're on macOS, you'll also need to add: `127.0.0.1     intcode.docker.localhost` to your `/etc/hosts` file.

### Dependencies (without Docker)

```bash
brew install luarocks openssl
luarocks init
./luarocks install --tree lua_modules --only-deps 07-dev-1.rockspec CRYPTO_INCDIR=/usr/local/opt/openssl/include/ OPENSSL_DIR=/usr/local/opt/openssl
```

## Run

> **WARNING**: These will take a long time, even with many instances of the intcode server. Might be a good idea to run them both at the same time in separate terminals.

```bash
./lua -l set_paths 07p1.run.lua
./lua -l set_paths 07p2.run.lua
```

## Run with Docker

Coming soon (maybe)