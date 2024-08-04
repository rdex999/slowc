#!/bin/bash

cargo run --features english -- example.slw
if (( $? != 0 )); then
	exit
fi

if [[ "$1" == "run" ]]; then
	until [ -f a.out ]
	do
		sleep 0.01
	done
	./a.out
	EXIT_CODE=$?
	rm a.out
	exit $EXIT_CODE
else
	gdb a.out \
		-ex "lay src" \
		-ex "lay regs" \
		-ex "br main" \
		-ex "run"
fi
rm a.out