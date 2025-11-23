GIT_ROOT := $(shell git rev-parse --show-toplevel)
OUTPUT_DIR := $(GIT_ROOT)/2024/build/lib
MODULE_NAME := solve01

ZIG := zig

all:
	mkdir -p $(OUTPUT_DIR)
	$(ZIG) build-lib -dynamic -fPIC solve.zig -O ReleaseSafe -lc -femit-bin=$(OUTPUT_DIR)/lib$(MODULE_NAME).so

clean:
	rm -f $(OUTPUT_DIR)/lib$(MODULE_NAME).so

