OUTPUT_DIR := ../../build/lib
MODULE_NAME := solve01

ZIG := zig

all:
	mkdir -p $(OUTPUT_DIR)
	$(ZIG) build-lib -dynamic -fPIC solve.zig -O ReleaseSafe -lc -femit-bin=$(OUTPUT_DIR)/lib$(MODULE_NAME).so

clean:
	rm -f $(OUTPUT_DIR)/lib$(MODULE_NAME).so

