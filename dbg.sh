#!/bin/bash

cargo run --features english -- example.slw
nasm -felf64 -g -o a.o /tmp/slowc_compiled.asm
ld -e main -o a.out a.o
gdb a.out\
	-ex "lay src" \
	-ex "lay regs" \
	-ex "br main" \
	-ex "run"

rm a.o a.out