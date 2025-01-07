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

### php-fpm

```

     data_received..................: 13 MB  1.3 MB/s
     data_sent......................: 5.4 MB 541 kB/s
     http_req_blocked...............: avg=2.48µs  min=900ns    med=1.99µs   max=611.45µs p(90)=2.6µs    p(95)=3.03µs
     http_req_connecting............: avg=126ns   min=0s       med=0s       max=437.26µs p(90)=0s       p(95)=0s
     http_req_duration..............: avg=1.43ms  min=288µs    med=714.29µs max=54.12ms  p(90)=881.36µs p(95)=975.7µs
       { expected_response:true }...: avg=1.43ms  min=288µs    med=714.29µs max=54.12ms  p(90)=881.36µs p(95)=975.7µs
     http_req_failed................: 0.00%  0 out of 67670
     http_req_receiving.............: avg=24.36µs min=7.99µs   med=21.4µs   max=732.77µs p(90)=31.11µs  p(95)=38.77µs
     http_req_sending...............: avg=6.35µs  min=2.4µs    med=5.42µs   max=757.61µs p(90)=7.17µs   p(95)=9.19µs
     http_req_tls_handshaking.......: avg=0s      min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=1.4ms   min=269.21µs med=685.1µs  max=54.09ms  p(90)=846.72µs p(95)=935.15µs
     http_reqs......................: 67670  6766.527655/s
     iteration_duration.............: avg=1.47ms  min=310.68µs med=746.85µs max=54.17ms  p(90)=918.39µs p(95)=1.01ms
     iterations.....................: 67670  6766.527655/s
     vus............................: 10     min=10         max=10
     vus_max........................: 10     min=10         max=10
```

### golang

```
     data_received..................: 23 MB  2.3 MB/s
     data_sent......................: 16 MB  1.6 MB/s
     http_req_blocked...............: avg=2.32µs   min=880ns    med=2.02µs   max=694.97µs p(90)=2.59µs   p(95)=3µs
     http_req_connecting............: avg=9ns      min=0s       med=0s       max=250.83µs p(90)=0s       p(95)=0s
     http_req_duration..............: avg=466.84µs min=115.73µs med=288.06µs max=37.2ms   p(90)=405.35µs p(95)=462.56µs
       { expected_response:true }...: avg=466.84µs min=115.73µs med=288.06µs max=37.2ms   p(90)=405.35µs p(95)=462.56µs
     http_req_failed................: 0.00%  0 out of 196929
     http_req_receiving.............: avg=20.65µs  min=6.7µs    med=17.84µs  max=1.22ms   p(90)=25.1µs   p(95)=31.93µs
     http_req_sending...............: avg=6.47µs   min=2.27µs   med=5.47µs   max=720.07µs p(90)=6.87µs   p(95)=8.17µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=439.72µs min=97.57µs  med=262.64µs max=37.17ms  p(90)=374.91µs p(95)=426.01µs
     http_reqs......................: 196929 19692.296138/s
     iteration_duration.............: avg=500.11µs min=136.28µs med=320.07µs max=37.23ms  p(90)=441.47µs p(95)=502.71µs
     iterations.....................: 196929 19692.296138/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10
```

### rust-redis

```
     data_received..................: 10 MB  1.0 MB/s
     data_sent......................: 11 MB  1.1 MB/s
     http_req_blocked...............: avg=1.81µs   min=860ns    med=1.59µs   max=503.61µs p(90)=2.33µs   p(95)=2.57µs
     http_req_connecting............: avg=13ns     min=0s       med=0s       max=290.65µs p(90)=0s       p(95)=0s
     http_req_duration..............: avg=722.37µs min=353.35µs med=711.7µs  max=1.93ms   p(90)=858.84µs p(95)=910.97µs
       { expected_response:true }...: avg=722.37µs min=353.35µs med=711.7µs  max=1.93ms   p(90)=858.84µs p(95)=910.97µs
     http_req_failed................: 0.00%  0 out of 132511
     http_req_receiving.............: avg=16.16µs  min=5.79µs   med=14.69µs  max=823.68µs p(90)=21.33µs  p(95)=23.83µs
     http_req_sending...............: avg=4.87µs   min=2.16µs   med=4.23µs   max=620.87µs p(90)=6.31µs   p(95)=7.05µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=701.33µs min=312.93µs med=691.29µs max=1.91ms   p(90)=836.45µs p(95)=886.74µs
     http_reqs......................: 132511 13250.205489/s
     iteration_duration.............: avg=748.66µs min=390.43µs med=737.45µs max=2.03ms   p(90)=886.75µs p(95)=940.53µs
     iterations.....................: 132511 13250.205489/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10
```

### rust-redis-deadpool

### rust-redis-deadpool-max-size

```
     data_received..................: 21 MB  2.1 MB/s
     data_sent......................: 21 MB  2.1 MB/s
     http_req_blocked...............: avg=1.89µs   min=830ns    med=1.77µs   max=493.75µs p(90)=2.35µs   p(95)=2.65µs
     http_req_connecting............: avg=5ns      min=0s       med=0s       max=210.44µs p(90)=0s       p(95)=0s
     http_req_duration..............: avg=355.2µs  min=146.76µs med=347.18µs max=1.6ms    p(90)=432.54µs p(95)=468.27µs
       { expected_response:true }...: avg=355.2µs  min=146.76µs med=347.18µs max=1.6ms    p(90)=432.54µs p(95)=468.27µs
     http_req_failed................: 0.00%  0 out of 257544
     http_req_receiving.............: avg=16.16µs  min=5.76µs   med=14.74µs  max=1.14ms   p(90)=20.53µs  p(95)=22.97µs
     http_req_sending...............: avg=5.17µs   min=2.19µs   med=4.74µs   max=783.52µs p(90)=6.32µs   p(95)=6.98µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=333.86µs min=133.78µs med=327.37µs max=1.57ms   p(90)=410.08µs p(95)=442.83µs
     http_reqs......................: 257544 25753.265172/s
     iteration_duration.............: avg=382.02µs min=164.38µs med=372.95µs max=1.68ms   p(90)=460.6µs  p(95)=498.54µs
     iterations.....................: 257544 25753.265172/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10
```