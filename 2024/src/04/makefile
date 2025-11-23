GIT_ROOT = $(shell git rev-parse --show-toplevel)
LIB_OUTPUT_DIR = $(GIT_ROOT)/2024/build/lib
MODULE_NAME = solve04
SRC = solve.swift
SWIFTC = swiftc
SWIFTFLAGS = -emit-library -module-name $(MODULE_NAME)

SWIFT_LIB = lib$(MODULE_NAME).so

all:
	mkdir -p $(LIB_OUTPUT_DIR)
	$(SWIFTC) $(SWIFTFLAGS) $(SRC) -o $(LIB_OUTPUT_DIR)/$(SWIFT_LIB)

clean:
	rm -f $(LIB_OUTPUT_DIR)/lib$(MODULE_NAME).so
