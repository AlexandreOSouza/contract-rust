#!/bin/bash

set -e 

RUSTFLAG='-C link-arg=s' cargo build -- target wasm32-unknow-unknow --release

cp target/wasm32-unknow-unknow/release/*./wasm ./res/

