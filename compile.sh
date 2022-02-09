#!/bin/sh
echo "compiling rust"
rustup target add wasm32-unknown-unknown
sudo apt install binaryen
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
wasm-opt -Oz ./target/wasm32-unknown-unknown/release/*.wasm -o ./contract.wasm
echo "finished"