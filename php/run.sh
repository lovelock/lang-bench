#!/usr/bin/env bash

podman build -t bench-redis-php .
podman run --cpus 2 --memory 2G --network=host --rm bench-redis-php