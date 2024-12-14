OUTPUT_DIR := ../../build/lib
MODULE_NAME := solve04
LIB_NAME := lib$(MODULE_NAME).so

NIM := nim
NIM_SRC := solve.nim

all: $(LIB_NAME)

$(LIB_NAME): $(NIM_SRC)
	mkdir -p $(OUTPUT_DIR)
	$(NIM) c --app:lib --out:$@ $<
	cp $(LIB_NAME) $(OUTPUT_DIR)

clean:
	rm $(LIB_NAME)
	rm -f $(OUTPUT_DIR)/$(LIB_NAME)

test:
	$(NIM) test test_$(NIM_SRC)

