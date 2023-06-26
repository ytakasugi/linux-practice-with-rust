#!/bin/bash

echo "/****************"
echo "BUILD"
echo "****************/"

cargo build --example ch01-01-inf-loop

echo "/****************"
echo "RUN"
echo "****************/"

sar -P 0 1 1

taskset -c 0 ./target/debug/examples/ch01-01-inf-loop &

sar -P 0 1 1

echo "/****************"
echo "KILL PROCESS"
echo "****************/"

TARGET_PID=`ps | grep ch01-01 | awk '{print $1}'`

kill ${TARGET_PID}