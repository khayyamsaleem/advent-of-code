## AOC 2019 Day 6

## Prereqs

A running instance of Neo4j

Create `.env` file with:

```
session=<aoc session cookie from browser>
bolt_uri=<uri of Neo4j bolt server || bolt://neo4j if you're using docker>
```

## Run

```bash
composer install
php index.php
```

## Run with Docker

```bash
docker-compose up neo4j
# wait for the bolt server to come up
docker-compose up solution
```
