## AOC 2019 Day 9

> **NOTE**: This isn't cheating.

## Prereqs

### Intcode Server
In the day 5 folder, run:

```bash
docker-compose up
```

Remember to add:

```
127.0.0.1    intcode.docker.localhost
```

to your `/etc/hosts` file.

### Environment
Create `.env` file with:

```
session=<aoc session cookie from browser>
```

### Dependencies

```bash
brew install httpie
```

## Run

```bash
zsh solution.zsh
```
