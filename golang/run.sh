#!/usr/bin/env bash

podman build -t bench-redis-go .
podman run --cpus 2 --memory 2G --network=host --rm bench-redis-go