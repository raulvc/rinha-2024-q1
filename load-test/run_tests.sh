#!/bin/bash

CWD=$(dirname "${BASH_SOURCE[0]}")

docker run --rm -it \
  --network host \
  -v "$CWD"/results:/gatling/results \
  raulvc/rinha-gatling \
  -s RinhaBackendCrebitosSimulation