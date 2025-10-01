#!/bin/bash

echo "=================================="
echo "RAZEN vs PYTHON LOOP BENCHMARK"
echo "=================================="
echo ""
echo "Test: 1,000,000 loop iterations"
echo ""

echo "--- Running Razen (3 times) ---"
for i in 1 2 3; do
    echo "Run $i:"
    time razen run benchmark_loop.rzn 2>&1 | grep -v "Starting\|Running\|Completed\|finished"
done

echo ""
echo "--- Running Python (3 times) ---"
for i in 1 2 3; do
    echo "Run $i:"
    time python3 benchmark_loop.py 2>&1 | grep -v "1000000"
done

echo ""
echo "=================================="
echo "BENCHMARK COMPLETE"
echo "=================================="
