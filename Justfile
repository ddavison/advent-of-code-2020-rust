WARMUP := '5'
BINARY_PATH := './target/release/aoc'

_default:
    just --choose

build:
    cargo build --release

day_1: build
    hyperfine --warmup {{WARMUP}} -- '{{BINARY_PATH}} day_1'

day_2: build
    hyperfine --warmup {{WARMUP}} -- '{{BINARY_PATH}} day_2'