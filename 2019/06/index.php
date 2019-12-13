<?php

require_once 'vendor/autoload.php';

use GraphAware\Neo4j\Client\ClientBuilder;

$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();

function getInput($uri) {
    $session_cookie = getenv("session");

    $response = \Httpful\Request::get($uri)
        ->addHeader("Cookie", "session={$session_cookie}")
        ->send();

    return explode(PHP_EOL,trim($response->body));
};


function buildGraph($neo4j_client, $orbits) {
    $stack = $neo4j_client->stack();
    $stack->push('CREATE CONSTRAINT ON (o:Object) ASSERT o.name IS UNIQUE');
    foreach ($orbits as $orbit) {
        [$orbitee, $orbiter] = explode(')', $orbit);
        $stack->push(
            'MERGE (m:Object {name: {orbiter}})
             MERGE (n:Object {name: {orbitee}})
             MERGE (n)-[:ORBITED_BY {cost: 1.0}]->(m)',
            ['orbiter' => $orbiter, 'orbitee' => $orbitee]
        );
    }
    $neo4j_client->runStack($stack);
};

function partOne($neo4j_client) {
    $QUERY = 'MATCH (n:Object {name:"COM"})
              CALL algo.shortestPath.deltaStepping.stream(n, "cost", 3.0)
              YIELD nodeId, distance
              MATCH (destination) WHERE id(destination) = nodeId
              RETURN SUM(distance)';
    return intval($neo4j_client->run($QUERY)->getRecord()->value('SUM(distance)'));
}

function partTwo($neo4j_client) {
    $QUERY = 'MATCH path=(:Object {name: "YOU"})-[*]-(:Object {name: "SAN"})
              RETURN length(path) - 2';
    return $neo4j_client->run($QUERY)->getRecord()->value("length(path) - 2");
}

function main() {

    $uri = "https://adventofcode.com/2019/day/6/input";

    $client = ClientBuilder::create()
        ->addConnection('bolt', getenv("bolt_uri"))
        ->setDefaultTimeout(30)
        ->build();
    buildGraph($client, getInput($uri));
    echo partOne($client);
    echo "\n";
    print_r (partTwo($client));
}

main();

?>
