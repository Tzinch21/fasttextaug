#!/bin/bash

set -e

exec python3 /app/bench_tests/ocr_single.py &
exec python3 /app/bench_tests/ocr_list.py &
exec python3 /app/bench_tests/keyboard_single.py &
exec python3 /app/bench_tests/keyboard_list.py &
exec python3 /app/bench_tests/random_char_single.py &
exec python3 /app/bench_tests/random_char_list.py &
exec python3 /app/bench_tests/random_word_single.py &
exec python3 /app/bench_tests/random_word_list.py
