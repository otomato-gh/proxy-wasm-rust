# Prime Authorizer - an Example of Building an Envoy WASM Filter with Rust

Based on the [proxy-wasm crate](https://crates.io/crates/proxy-wasm/0.1.0)

The filter is ready to be built and verified with the enclosed docker-compose environment.

## Building and running:

1. clone this repo
2. rustup target add wasm32-unknown-unknown, if needed for cross compiling
3. `cargo build --target=wasm32-unknown-unknown --release`
4. `docker-compose up --build`

## What the Filter Does
Each request directed to our service needs to be authorized by sending a token which is then checked for validity by the filter. If the token is validated - the request is passed on to the service. Otherwise - 403 response is returned to the caller. 
The validity check for the token is quite dumb - we check if the token is a prime number.

## Testing it Works
```bash
curl  -H "token":"323232" 0.0.0.0:18000
Access forbidden.

curl  -H "token":"32323" 0.0.0.0:18000
"Welcome to WASM land"
```

Read this [blog post](https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/) for more details on Envoy, WASM and Rust.
