LIB_OUTPUT_DIR = ../../build/lib
MODULE_NAME = solve03
CARGO = cargo
TARGET = target/release
RUSTLIB = $(LIB_OUTPUT_DIR)/lib$(MODULE_NAME).so

all:
	mkdir -p $(LIB_OUTPUT_DIR)
	$(CARGO) build --release --lib
	cp $(TARGET)/libaoc_2024_day_03.so $(RUSTLIB)

test:
	$(CARGO) test

clean:
	$(CARGO) clean
	rm -f $(RUSTLIB)
