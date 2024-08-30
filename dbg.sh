#!/bin/bash

cargo run --features english -- example.slw
if (( $? != 0 )); then
	exit
fi

until [ -f a.out ]
do
	sleep 0.01
done

if [[ "$1" == "run" ]]; then
	./a.out
	EXIT_CODE=$?
	echo -e "\nEXITED WITH: $EXIT_CODE"
else
	gdb a.out \
		-ex "lay src" \
		-ex "lay regs" \
		-ex "br main" \
		-ex "run"
fi

rm a.out
exit $EXIT_CODE