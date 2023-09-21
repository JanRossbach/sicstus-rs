export LD_LIBRARY_PATH := $(shell pwd)/target/debug

.PHONY: test

build:
	cargo build

ex.so: build
	splfr ex.pl ex.c --verbose --keep -o ex.so
	gcc -shared    -Wl,--version-script=ex_glue.mapfile ex.o ex_glue.o -o ex.so  -ldl -lm -lpthread -lrt -lsatsolver_kissat -Ltarget/debug

test: ex.so
	sicstus -l ex -- -Ltarget/debug

satsolver.so: build
	splfr satsolver.pl satsolver.c --keep -o satsolver.so
	gcc -shared    -Wl,--version-script=satsolver_glue.mapfile satsolver.o satsolver_glue.o -o satsolver.so  -ldl -lm -lpthread -lrt -lsatsolver_kissat -Ltarget/debug

satsolver: satsolver.so
	sicstus -l satsolver -- -Ltarget/debug

clean:
	rm -f *.so *.o *_glue*
