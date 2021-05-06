#!/bin/bash

set -eu

file_number="0000"
if [ $# == 1 ]; then
    file_number=$1
fi

echo "run test ${file_number}.txt"
cargo run --bin a < "tools/in/${file_number}.txt" > "tools/out/${file_number}.txt"
cd tools
cargo run --release --bin my_vis "in/${file_number}.txt" "out/${file_number}.txt"
# open out.svg
open vis.html
