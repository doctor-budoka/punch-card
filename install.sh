#!/bin/bash
cargo build --release
cp ./target/release/punch-card /usr/local/bin/punch