## AOC 2019 Day 5

## Prereqs
Create `.env` file with:

```
session=<aoc session cookie from browser>
```

## Run

```bash
raco pkg install
raco exe -o ham-aocd5 main.rkt
./ham-aocd5
```

## Run with Docker

```bash
docker build . -f Dockerfile.solution -t ham-aocd5
docker run ham-aocd5
```
