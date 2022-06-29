#!/bin/sh

echo "" > results2.txt
OUTPUT=$(realpath results2.txt)
export HACK_FUEL=10000000000

###

cd ..

git checkout Cargo.toml
echo '[patch.crates-io]' >> Cargo.toml
echo 'wasmtime = { git = "https://github.com/bytecodealliance/wasmtime.git", rev = "f19d8cc8510535b03eb92e6eb9a643bd47f5314e" }' >> Cargo.toml

export HACK_CONSUME_FUEL=0
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-vanilla --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_no_fuel/g" >> $OUTPUT
cd ../../..

export HACK_CONSUME_FUEL=1
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-vanilla --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_vanilla_fuel/g" >> $OUTPUT
cd ../../..

export HACK_CONSUME_FUEL=1
git checkout Cargo.toml
echo '[patch.crates-io]' >> Cargo.toml
echo 'wasmtime = { git = "https://github.com/pepyakin/wasmtime.git", branch = "pep-pinned-fuel" }' >> Cargo.toml

export HACK_SIGNAL_INTERVAL=10000
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_10000us/g" >> $OUTPUT
cd ../../..

export HACK_SIGNAL_INTERVAL=1000
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_1000us/g" >> $OUTPUT
cd ../../..

export HACK_SIGNAL_INTERVAL=500
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_500us/g" >> $OUTPUT
cd ../../..

export HACK_SIGNAL_INTERVAL=250
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_250us/g" >> $OUTPUT
cd ../../..

export HACK_SIGNAL_INTERVAL=100
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_100us/g" >> $OUTPUT
cd ../../..

export HACK_SIGNAL_INTERVAL=50
cd client/executor/benches
cargo bench --features wasmtime,sc-executor-wasmtime/fuel-check-async --bench bench -- --output-format=bencher wasmi_regex_redux_from_test_runtime_with_pooling_cow_fresh_on_1_threads | sed "s/with_pooling_cow_fresh/with_async_fuel_50us/g" >> $OUTPUT
cd ../../..
