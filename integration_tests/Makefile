export LD_LIBRARY_PATH = $(shell pwd)/target/debug

EXT_NAME=integration_tests
CFLAGS=-ldl -lm -lpthread -lrt -l$(EXT_NAME) -Ltarget/debug

run: $(EXT_NAME).so
	@sicstus -l $(EXT_NAME) --goal "rust_main, halt."

$(EXT_NAME).so: $(EXT_NAME)_glue.o
	@gcc -shared    -Wl,--version-script=$(EXT_NAME)_glue.mapfile $(EXT_NAME).o $(EXT_NAME)_glue.o -o $(EXT_NAME).so $(CFLAGS)

$(EXT_NAME)_glue.o: $(EXT_NAME).pl $(EXT_NAME).c
	@splfr $(EXT_NAME).pl $(EXT_NAME).c --keep -o $(EXT_NAME).so

$(EXT_NAME).c: build

build:
	@cargo build

clean:
	@cargo clean
	@rm -f *.so *.o *_glue* $(EXT_NAME).c
