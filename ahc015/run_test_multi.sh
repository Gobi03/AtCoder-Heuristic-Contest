#/bin/bash

set -eu

cargo build --release

for i in `seq 0 10`
do
    test_num=`printf %04d ${i}`
    echo "case: #${test_num}"
    ../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
    cd tools
    cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"
    echo ""
    cd ..
done
