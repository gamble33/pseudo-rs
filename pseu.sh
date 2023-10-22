#!/bin/zsh
cargo run examples/test.pseudo 2>/dev/null
gcc target.c
./a.out
