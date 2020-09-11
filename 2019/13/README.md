## AOC 2019 Day 10

## Prereqs

### Ruby
```bash
brew install ruby
```

### Environment
Create `.env` file with:

```
session=<aoc session cookie from browser>
intcode_server_uri=<server uri for day 5 intcode server>
```

## Run with Docker

Either use `docker-compose` to get a version of the intcode server on the spot (it's on docker hub now!) or spawn an instance of the intcode server from day 5's README instructions.

## Run without Docker

Still need an intcode server, but can just run the following commands after it's set up and added to the `.env` file:

```bash
brew install ruby
gem install bundler
bundle
ruby main.rb
```
