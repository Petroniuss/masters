#!/bin/bash


function single_peerset_benchmark() {
  BENCHMARK_FILE=./benchmark/single_peerset_local.csv
  echo "peers_num, iterations, deployment_gas_used, deployment_time, changes_gas_used, changes_time" > $BENCHMARK_FILE

  LOG_FILE="output.log"
  function benchmark_iter() {
    docker-compose up -d

    export PEERS_NUM="$1"
    cargo test --manifest-path=./organisation/Cargo.toml \
      --color=always --package organisation \
      --test single_peerset_benchmark single_peerset_benchmark \
      -- --format=json --exact -Z unstable-options --show-output \
      2>&1 | tee /dev/tty > $LOG_FILE 2>&1

    OUTPUT=$(cat $LOG_FILE)
    CONTAINER_ID=$(docker ps --filter "name=masters-anvil" --format "{{.ID}}")

    DEPLOYMENT_TIME=$(echo "$OUTPUT" | grep -oE 'Peerset created, time: [0-9]+\.[0-9]+s' | awk '{print $4}')
    DEPLOYMENT_GAS=$(docker logs "$CONTAINER_ID" | grep -A 1 'Contract created: ' | tail -1 | awk '{print $7}')

    CHANGES_TIME=$(echo "$OUTPUT" | grep -oE 'Time elapsed: [0-9]+\.[0-9]+s' | awk '{print $3}')
    SUM=$(docker logs "$CONTAINER_ID" | grep "Gas used:" | awk '{ sum += $NF } END { print sum }')
    CHANGES_GAS=$((SUM - DEPLOYMENT_GAS))

    # 10, 1, 1941417, Peerset created, time: 7.062907s, 725980, Time elapsed: 13.997478s

    echo "Peers num: $PEERS_NUM"
    echo "Iterations: $ITER_NUM"
    echo "Deployment gas used: $DEPLOYMENT_GAS"
    echo "Deployment time: $DEPLOYMENT_TIME"
    echo "Changes gas used: $CHANGES_GAS"
    echo "Changes time: $CHANGES_TIME"
    echo "$PEERS_NUM, $ITER_NUM, $DEPLOYMENT_GAS, $DEPLOYMENT_TIME, $CHANGES_GAS, $CHANGES_TIME" >> $BENCHMARK_FILE

    docker-compose down
  }

  export ITER_NUM="1"
  for i in 2 3 5 10
  do
    benchmark_iter $i
  done
}

function cross_peerset_benchmark() {
  BENCHMARK_FILE=./benchmark/cross_peerset_local.csv
  echo "peers_num, iterations, deployment_gas_used, deployment_time, changes_gas_used, changes_time" > $BENCHMARK_FILE

  LOG_FILE="output.log"
  function benchmark_iter() {
    docker-compose up -d

    export PEERS_NUM="$1"
    cargo test --manifest-path=./organisation/Cargo.toml \
      --color=always --package organisation \
      --test cross_peerset_benchmark cross_peerset_benchmark \
      -- --format=json --exact -Z unstable-options --show-output \
      2>&1 | tee /dev/tty > $LOG_FILE 2>&1

    OUTPUT=$(cat $LOG_FILE)
    CONTAINER_ID=$(docker ps --filter "name=masters-anvil" --format "{{.ID}}")

    DEPLOYMENT_TIME=$(echo "$OUTPUT" | grep -oE 'Peerset created, time: [0-9]+\.[0-9]+s' \
      | awk '{sum += substr($NF, 1, length($NF)-1)} END {print sum}')
    DEPLOYMENT_TIME="${DEPLOYMENT_TIME}s"

    DEPLOYMENT_GAS=$(docker logs "$CONTAINER_ID" | grep "Gas used" | head -n 2 | awk '{sum += $7} END {print sum}')

    CHANGES_TIME=$(echo "$OUTPUT" | grep -oE 'Time elapsed: [0-9]+\.[0-9]+s' | awk '{print $3}')
    SUM=$(docker logs "$CONTAINER_ID" | grep "Gas used:" | awk '{ sum += $NF } END { print sum }')
    CHANGES_GAS=$((SUM - DEPLOYMENT_GAS))


    echo "Peers num: $PEERS_NUM"
    echo "Iterations: $ITER_NUM"
    echo "Deployment gas used: $DEPLOYMENT_GAS"
    echo "Deployment time: $DEPLOYMENT_TIME"
    echo "Changes gas used: $CHANGES_GAS"
    echo "Changes time: $CHANGES_TIME"
    echo "$PEERS_NUM, $ITER_NUM, $DEPLOYMENT_GAS, $DEPLOYMENT_TIME, $CHANGES_GAS, $CHANGES_TIME" >> $BENCHMARK_FILE

    docker-compose down
  }

  export ITER_NUM="1"
  for i in 2 3 5 10
  do
    benchmark_iter $i
  done
}

single_peerset_benchmark
cross_peerset_benchmark
