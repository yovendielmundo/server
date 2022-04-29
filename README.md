# Server

## About
The `server` is an implementation of a http server written in Rust.


## How to start developing

### Run
```shell
cargo run
```

### Test
```shell
cargo test
```

### Deploy (Docker)
```shell
cargo build --release
docker build -t $USER/server:latest .
docker run -it --rm --name server $USER/server:latest
```
