#!/bin/bash

set -e

exec python3 /app/bench_tests/ocr_single.py &
exec python3 /app/bench_tests/ocr_list.py 
