server(debian)

```bash
podman run --rm -d --network=host --name=server debian --cpus 2 --memory 2G
```

redis

```bash
podman run --rm -d --network=host --name=redis redis --cpus 2 --memory 2G
```

cadvisor

```bash
podman run -d --name cadvisor \
  --volume /:/rootfs:ro \
  --volume /dev/disk/:/dev/disk:ro \
  --volume /etc/machine-id:/etc/machine-id:ro \
  --volume /sys:/sys:ro \
  --volume /sys/fs/cgroup:/sys/fs/cgroup:ro \
  --volume /var/lib/containers:/var/lib/containers:ro \
  --volume /var/lib/dbus/machine-id:/var/lib/dbus/machine-id:ro \
  --volume /var/run:/var/run:rw \
  --privileged \
  -p 127.0.0.1:8081:8080 \
  gcr.io/cadvisor/cadvisor:latest
```
