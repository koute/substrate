#!/bin/sh
HACK_CONSUME_FUEL=1 HACK_SIGNAL_INTERVAL=10 HACK_FUEL=100000000 cargo bench --profile dev --features wasmtime -- return_value_from_test_runtime_with_recreate_instance_vanilla_on_1_threads