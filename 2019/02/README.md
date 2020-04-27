# AOC 2019 Day 2

## Prereqs
Create `.env` file with:

```
session=<aoc session cookie from browser>
```

in the `aoc_day_2` directory

## Build
`cd aoc_day_2`
`nimble build`

## Run

`./aoc_day_2 1` runs part one

`./aoc_day_2 2 <target>` for part two

## Run with Docker

```bash
cd aoc_day_2
docker build . -t ham-aocd2
docker run ham-aocd2
```
