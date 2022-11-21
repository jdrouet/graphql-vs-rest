#!/bin/bash

export USER_ID=$(id -u)

docker-compose build

function execute_one() {
    docker-compose rm --stop --force --volumes
    docker-compose up -d server-$1-api
    sleep 10
    docker-compose run runner-$1-api > results/$1-00$2.txt

    ITERATIONS=$(jq .metrics.iterations.count results/export.json)
    DATA_RECEIVED=$(jq .metrics.data_received.count results/export.json)
    DATA_SENT=$(jq .metrics.data_sent.count results/export.json)
    echo "$1,$2,$ITERATIONS,$DATA_RECEIVED,$DATA_SENT" >> results/summary.csv
}

function execute() {
    for i in {1..9}; do
        execute_one $1 $i
    done
}

echo "service,execution,iterations,data_received,data_sent" > results/summary.csv
execute graphql
execute rest
