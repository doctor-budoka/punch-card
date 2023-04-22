#!/bin/bash
cargo build --release
cp ./target/release/punch /usr/local/bin/punch