export LD_LIBRARY_PATH := $(shell pwd)/target/debug

.PHONY: test

build:
	cargo build

ex.so: build
	splfr ex.pl ex.c --verbose --keep -o ex.so
	gcc -shared    -Wl,--version-script=ex_glue.mapfile ex.o ex_glue.o -o ex.so  -ldl -lm -lpthread -lrt -lsicstus_rs -Ltarget/debug

test: ex.so
	sicstus -l ex -- -Ltarget/debug

clean:
	rm -f *.so *.o *_glue*
