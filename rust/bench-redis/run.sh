#!/usr/bin/env bash

podman build -t bench-redis .
podman run --cpus 2 --memory 2G --network=host --rm -v $(pwd):/usr/src/myapp:Z bench-redis