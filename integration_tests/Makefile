EXT_NAME=integration_tests
LIB = lib$(EXT_NAME).a

splfr = $(SICSTUSDIR)/bin/splfr
sicstus = $(SICSTUSDIR)/bin/sicstus

INCLUDEFLAGS = -I"$(SICSTUSDIR)/include" -I"." -I"$(pwd)/target/debub/"
SICSTUSFLAGS = -g -O2 -pipe -D_REENTRANT -D_GNU_SOURCE -D_FILE_OFFSET_BITS=64 -fPIC -DSPDLL  -DSP_RESNAME=integration_0x5Ftests -DSICSTUS_TARGET_VERSION=40900
LDFLAGS += -Wl,--version-script=$(EXT_NAME)_glue.mapfile
DYLIBS=-ldl -lm -lpthread -lrt

.PHONY: all run clean

all: $(EXT_NAME).so

run: $(EXT_NAME).so
	@$(sicstus) -l $(EXT_NAME) --goal "rust_main, halt."

$(EXT_NAME).so: $(EXT_NAME)_glue.o
	@gcc -shared $(LDFLAGS) $(EXT_NAME).o $(EXT_NAME)_glue.o $(LIB) -o $(EXT_NAME).so $(DYLIBS)

$(EXT_NAME)_glue.o: $(EXT_NAME)_glue.c $(EXT_NAME)_glue.h $(EXT_NAME).o
	@gcc $(INCLUDEFLAGS) $(SICSTUSFLAGS) -c integration_tests_glue.c -o integration_tests_glue.o

$(EXT_NAME).o: $(EXT_NAME).c
	@gcc $(INCLUDEFLAGS) $(SICSTUSFLAGS) -c $(EXT_NAME).c -o integration_tests.o

$(EXT_NAME)_glue.c $(EXT_NAME)_glue.h: $(EXT_NAME).pl $(EXT_NAME).c $(LIB)
	@$(splfr) $(EXT_NAME).pl $(EXT_NAME).c $(LIB) --nocompile

$(EXT_NAME).c: build

$(LIB): build
	@echo "Copying library"
	@rm -f $(LIB)
	@mv target/debug/lib$(EXT_NAME).a $(LIB)

build:
	@cargo build

clean: cargo-clean
	@rm -f *.so *.o *_glue* *.a $(EXT_NAME).c

cargo-clean:
	@cargo clean
