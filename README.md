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
         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: script.js
        output: -

     scenarios: (100.00%) 1 scenario, 10 max VUs, 40s max duration (incl. graceful stop):
              * default: 10 looping VUs for 10s (gracefulStop: 30s)


     data_received..................: 13 MB  1.3 MB/s
     data_sent......................: 5.6 MB 557 kB/s
     http_req_blocked...............: avg=2.45µs  min=950ns    med=2.02µs   max=829.08µs p(90)=2.57µs  p(95)=3µs    
     http_req_connecting............: avg=152ns   min=0s       med=0s       max=650.54µs p(90)=0s      p(95)=0s     
     http_req_duration..............: avg=1.39ms  min=430.93µs med=844.61µs max=45.28ms  p(90)=1.14ms  p(95)=1.3ms  
       { expected_response:true }...: avg=1.39ms  min=430.93µs med=844.61µs max=45.28ms  p(90)=1.14ms  p(95)=1.3ms  
     http_req_failed................: 0.00%  0 out of 69567
     http_req_receiving.............: avg=24.66µs min=7.79µs   med=22.1µs   max=1.33ms   p(90)=30.65µs p(95)=36.81µs
     http_req_sending...............: avg=6.52µs  min=2.33µs   med=5.66µs   max=672.77µs p(90)=7.23µs  p(95)=9.01µs 
     http_req_tls_handshaking.......: avg=0s      min=0s       med=0s       max=0s       p(90)=0s      p(95)=0s     
     http_req_waiting...............: avg=1.36ms  min=414.98µs med=814.59µs max=45.24ms  p(90)=1.11ms  p(95)=1.26ms 
     http_reqs......................: 69567  6956.10171/s
     iteration_duration.............: avg=1.42ms  min=450.79µs med=877.42µs max=45.31ms  p(90)=1.18ms  p(95)=1.34ms 
     iterations.....................: 69567  6956.10171/s
     vus............................: 10     min=10         max=10
     vus_max........................: 10     min=10         max=10


running (10.0s), 00/10 VUs, 69567 complete and 0 interrupted iterations
default ✓ [======================================] 10 VUs  10s
```

### golang

```
         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: script.js
        output: -

     scenarios: (100.00%) 1 scenario, 10 max VUs, 40s max duration (incl. graceful stop):
              * default: 10 looping VUs for 10s (gracefulStop: 30s)


     data_received..................: 22 MB  2.2 MB/s
     data_sent......................: 15 MB  1.4 MB/s
     http_req_blocked...............: avg=2.38µs   min=840ns    med=2.04µs   max=577.39µs p(90)=2.63µs   p(95)=3.04µs  
     http_req_connecting............: avg=12ns     min=0s       med=0s       max=364.1µs  p(90)=0s       p(95)=0s      
     http_req_duration..............: avg=511.69µs min=118.21µs med=314.96µs max=32.15ms  p(90)=439.44µs p(95)=501.19µs
       { expected_response:true }...: avg=511.69µs min=118.21µs med=314.96µs max=32.15ms  p(90)=439.44µs p(95)=501.19µs
     http_req_failed................: 0.00%  0 out of 180665
     http_req_receiving.............: avg=21.01µs  min=6.23µs   med=18.19µs  max=1.24ms   p(90)=25.47µs  p(95)=32.91µs 
     http_req_sending...............: avg=6.71µs   min=2.29µs   med=5.64µs   max=1.29ms   p(90)=6.97µs   p(95)=8.27µs  
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s      
     http_req_waiting...............: avg=483.97µs min=100.32µs med=288.92µs max=32.13ms  p(90)=408.49µs p(95)=464.67µs
     http_reqs......................: 180665 18065.812211/s
     iteration_duration.............: avg=545.65µs min=143.17µs med=347.65µs max=32.19ms  p(90)=476.57µs p(95)=542.68µs
     iterations.....................: 180665 18065.812211/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10


running (10.0s), 00/10 VUs, 180665 complete and 0 interrupted iterations
default ✓ [======================================] 10 VUs  10s
```

### rust-redis

```
         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: script.js
        output: -

     scenarios: (100.00%) 1 scenario, 10 max VUs, 40s max duration (incl. graceful stop):
              * default: 10 looping VUs for 10s (gracefulStop: 30s)


     data_received..................: 9.3 MB 933 kB/s
     data_sent......................: 9.6 MB 957 kB/s
     http_req_blocked...............: avg=2.14µs   min=830ns    med=1.91µs   max=698.82µs p(90)=2.35µs   p(95)=2.73µs  
     http_req_connecting............: avg=18ns     min=0s       med=0s       max=255.55µs p(90)=0s       p(95)=0s      
     http_req_duration..............: avg=798.78µs min=302.51µs med=782.71µs max=1.9ms    p(90)=952.91µs p(95)=1.02ms  
       { expected_response:true }...: avg=798.78µs min=302.51µs med=782.71µs max=1.9ms    p(90)=952.91µs p(95)=1.02ms  
     http_req_failed................: 0.00%  0 out of 119596
     http_req_receiving.............: avg=18.27µs  min=6.35µs   med=16.42µs  max=944.81µs p(90)=22.27µs  p(95)=25.84µs 
     http_req_sending...............: avg=5.93µs   min=2.4µs    med=5.19µs   max=695.99µs p(90)=6.43µs   p(95)=7.81µs  
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s      
     http_req_waiting...............: avg=774.57µs min=270.9µs  med=759.52µs max=1.87ms   p(90)=927.43µs p(95)=992.87µs
     http_reqs......................: 119596 11958.765696/s
     iteration_duration.............: avg=829.38µs min=367.02µs med=812.31µs max=2.58ms   p(90)=984.33µs p(95)=1.05ms  
     iterations.....................: 119596 11958.765696/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10


running (10.0s), 00/10 VUs, 119596 complete and 0 interrupted iterations
default ✓ [======================================] 10 VUs  10s
```

### rust-redis-deadpool
```
         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: script.js
        output: -

     scenarios: (100.00%) 1 scenario, 10 max VUs, 40s max duration (incl. graceful stop):
              * default: 10 looping VUs for 10s (gracefulStop: 30s)


     data_received..................: 18 MB  1.8 MB/s
     data_sent......................: 18 MB  1.8 MB/s
     http_req_blocked...............: avg=2.03µs   min=760ns    med=1.87µs   max=694.28µs p(90)=2.4µs    p(95)=2.76µs  
     http_req_connecting............: avg=7ns      min=0s       med=0s       max=216.37µs p(90)=0s       p(95)=0s      
     http_req_duration..............: avg=411.92µs min=156.51µs med=394.45µs max=3.31ms   p(90)=533.35µs p(95)=597.06µs
       { expected_response:true }...: avg=411.92µs min=156.51µs med=394.45µs max=3.31ms   p(90)=533.35µs p(95)=597.06µs
     http_req_failed................: 0.00%  0 out of 223432
     http_req_receiving.............: avg=17.21µs  min=5.54µs   med=15.34µs  max=1.28ms   p(90)=21.54µs  p(95)=24.85µs 
     http_req_sending...............: avg=5.73µs   min=2.2µs    med=5.04µs   max=1.02ms   p(90)=6.46µs   p(95)=7.31µs  
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s      
     http_req_waiting...............: avg=388.97µs min=140.89µs med=373.06µs max=3ms      p(90)=507.3µs  p(95)=567.85µs
     http_reqs......................: 223432 22342.253886/s
     iteration_duration.............: avg=440.94µs min=177.51µs med=422.39µs max=3.34ms   p(90)=565.25µs p(95)=631.81µs
     iterations.....................: 223432 22342.253886/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10


running (10.0s), 00/10 VUs, 223432 complete and 0 interrupted iterations
default ✓ [======================================] 10 VUs  10s
```

### rust-redis-deadpool-max-size

```
         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: script.js
        output: -

     scenarios: (100.00%) 1 scenario, 10 max VUs, 40s max duration (incl. graceful stop):
              * default: 10 looping VUs for 10s (gracefulStop: 30s)


     data_received..................: 19 MB  1.8 MB/s
     data_sent......................: 18 MB  1.8 MB/s
     http_req_blocked...............: avg=2.02µs   min=840ns    med=1.84µs   max=863.82µs p(90)=2.36µs   p(95)=2.7µs   
     http_req_connecting............: avg=5ns      min=0s       med=0s       max=250.47µs p(90)=0s       p(95)=0s      
     http_req_duration..............: avg=402.99µs min=139.44µs med=388.32µs max=1.78ms   p(90)=521.06µs p(95)=578.26µs
       { expected_response:true }...: avg=402.99µs min=139.44µs med=388.32µs max=1.78ms   p(90)=521.06µs p(95)=578.26µs
     http_req_failed................: 0.00%  0 out of 228363
     http_req_receiving.............: avg=16.8µs   min=5.47µs   med=14.98µs  max=1.37ms   p(90)=21.09µs  p(95)=24.16µs 
     http_req_sending...............: avg=5.56µs   min=2.17µs   med=4.95µs   max=1.02ms   p(90)=6.36µs   p(95)=7.15µs  
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s       p(90)=0s       p(95)=0s      
     http_req_waiting...............: avg=380.63µs min=125.5µs  med=367.2µs  max=1.66ms   p(90)=495.98µs p(95)=549.82µs
     http_reqs......................: 228363 22835.585221/s
     iteration_duration.............: avg=431.39µs min=159.13µs med=415.62µs max=2ms      p(90)=552.04µs p(95)=611.55µs
     iterations.....................: 228363 22835.585221/s
     vus............................: 10     min=10          max=10
     vus_max........................: 10     min=10          max=10


running (10.0s), 00/10 VUs, 228363 complete and 0 interrupted iterations
default ✓ [======================================] 10 VUs  10s
```