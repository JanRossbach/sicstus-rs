export LD_LIBRARY_PATH = $(shell pwd)/target/debug

EXT_NAME=integration_tests
CFLAGS=-ldl -lm -lpthread -lrt

run: $(EXT_NAME).so
	@sicstus -l $(EXT_NAME) --goal "rust_main, halt."

$(EXT_NAME).so: $(EXT_NAME)_glue.o
	@gcc -shared    -Wl,--version-script=$(EXT_NAME)_glue.mapfile $(EXT_NAME).o $(EXT_NAME)_glue.o libintegration_tests.a -o $(EXT_NAME).so $(CFLAGS)

$(EXT_NAME)_glue.o: $(EXT_NAME)_glue.c $(EXT_NAME)_glue.h libintegration_tests.a
	@gcc -I"/home/jan/.local/share/sicstus4.8.0/include" -I"." -g -O2 -pipe -D_REENTRANT -D_GNU_SOURCE -D_FILE_OFFSET_BITS=64 -fPIC -DSPDLL  -DSP_RESNAME=integration_0x5Ftests -DSICSTUS_TARGET_VERSION=40800 -c integration_tests_glue.c libintegration_tests.a -o integration_tests_glue.o

$(EXT_NAME)_glue.h $(EXT_NAME)_glue.c: $(EXT_NAME).pl $(EXT_NAME).c
	@splfr $(EXT_NAME).pl $(EXT_NAME).c --keep -o $(EXT_NAME).so

$(EXT_NAME).c: build

libintegration_tests.a:
	@cargo build
	@mv target/debug/lib$(EXT_NAME).a libintegration_tests.a

build:
	@cargo build

clean:
	@rm -f *.so *.o *_glue* *.a
