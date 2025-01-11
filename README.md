# CWASI Containerd shim


## Prerequisites

* Rust 
* Containerd 
* Wasmedge -v 0.11.2
* Cri-tools for execution

## Installation
```
cargo build --release
```

Copy binary to $PATH
```
sudo cp target/release/containerd-shim-cwasi-v1 /usr/local/bin/containerd-shim-cwasi-v1
```

## Usage


```
docker build -t func_a . \ 
&& docker save -o func_a.tar func_a:latest \
&& sudo ctr -n mysp images rm docker.io/library/funca:latest \
&& sudo ctr -n mysp images import funca.tar


sudo ctr -n mysp run --rm --runtime=io.containerd.cwasi.v1 \
--net-host=true \
--env STORAGE_IP=127.0.0.1:9999 \
--env REDIS_IP=127.0.0.1 \
--env FUNCTIONS_NUM=1 \
docker.io/keniack/func_a:latest fa \
/func_a.wasm /func_b.wasm file_1M.txt



sudo ctr -n mysp run --rm --runtime=io.containerd.cwasi.v1 \
--net-host=true \
--annotation  cwasi.secondary.function=true \
docker.io/keniack/func_b:latest fb \
/func_b.wasm

```
