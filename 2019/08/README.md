## AOC 2019 Day 8

## Prereqs

### Environment
Create `.env` file with:

```
session=<aoc session cookie from browser>
```

in `aoc-day8-solution`

### Dependencies (without Docker)

```bash
brew install stack
stack setup
```

## Run

```bash
cd aoc-day8-solution
stack run
```

## Run with Docker

```bash
cd aoc-day8-solution
docker build . -t ham-aocd8
docker run ham-aocd8
```