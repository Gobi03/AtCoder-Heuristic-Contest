#!/bin/bash

set -eu

for file_path in $(ls tools/in/); do
    echo "run test ${file_path}"
    cargo run --bin a < "tools/in/${file_path}" > "tools/out/${file_path}"
done
cd tools
cargo run --release --bin vis in/0000.txt out/0000.txt
