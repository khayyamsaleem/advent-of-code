<?php

require_once 'vendor/autoload.php';

use Laudis\Neo4j\ClientBuilder;
use Laudis\Neo4j\Contracts\TransactionInterface;

$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();

function getInput($uri) {
    $session_cookie = $_ENV["session"];

    $context = stream_context_create([
        'http' => [
            'method' => 'GET',
            'header' => "Cookie: session={$session_cookie}\r\n" .
                "User-Agent: github.com/khayyamsaleem/advent-of-code by hello@khayyam.me\r\n"
        ]
    ]);
    $body = file_get_contents($uri, false, $context);

    return explode(PHP_EOL, trim($body));
};


function buildGraph($neo4j_client, $orbits) {
    $neo4j_client->run('CREATE CONSTRAINT ON (o:Object) ASSERT o.name IS UNIQUE');
    $neo4j_client->writeTransaction(function (TransactionInterface $tsx) use ($orbits) {
        foreach ($orbits as $orbit) {
            [$orbitee, $orbiter] = explode(')', $orbit);
            $tsx->run(
                'MERGE (m:Object {name: $orbiter})
                 MERGE (n:Object {name: $orbitee})
                 MERGE (n)-[:ORBITED_BY {cost: 1.0}]->(m)',
                ['orbiter' => $orbiter, 'orbitee' => $orbitee]
            );
        }
    });
};

function partOne($neo4j_client) {
    $QUERY = 'MATCH (n:Object {name:"COM"})
              CALL algo.shortestPath.deltaStepping.stream(n, "cost", 3.0)
              YIELD nodeId, distance
              MATCH (destination) WHERE id(destination) = nodeId
              RETURN SUM(distance) AS total';
    return intval($neo4j_client->run($QUERY)->first()->get('total'));
}

function partTwo($neo4j_client) {
    $QUERY = 'MATCH path=(:Object {name: "YOU"})-[*]-(:Object {name: "SAN"})
              RETURN length(path) - 2 AS hops';
    return $neo4j_client->run($QUERY)->first()->get('hops');
}

function main() {

    $uri = "https://adventofcode.com/2019/day/6/input";

    $client = ClientBuilder::create()
        ->withDriver('bolt', $_ENV["bolt_uri"])
        ->withDefaultDriver('bolt')
        ->build();
    buildGraph($client, getInput($uri));
    echo partOne($client);
    echo "\n";
    print_r (partTwo($client));
}

main();

?>
