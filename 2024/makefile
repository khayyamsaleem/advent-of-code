SRC_DIR := src
BUILD_DIR := build
LIB_DIR := $(BUILD_DIR)/lib
BIN_DIR := $(BUILD_DIR)/bin
CACHE_DIR := .zig-cache

# Automatically find all module directories under src/ that match the pattern [0-9][0-9]
MODULES := $(shell ls -1d $(SRC_DIR)/[0-9][0-9] 2>/dev/null | xargs -n1 basename)

ZIG := zig
ZIG_BUILD_FLAGS := -p $(BUILD_DIR) -freference-trace

.PHONY: all clean modules run

all: modules
	$(ZIG) build $(ZIG_BUILD_FLAGS)

modules:
	@echo "Building modules: $(MODULES)"
	@for module in $(MODULES); do \
		$(MAKE) -C $(SRC_DIR)/$$module; \
	done

run:
	@if [ -z "$(ARGS)" ]; then \
		echo "Error: ARGS must be set to the module number (e.g., ARGS=01)"; \
		exit 1; \
	fi
	@if echo "$(ARGS)" | grep -E '^[0-9][0-9]$$' >/dev/null; then \
		module=$(SRC_DIR)/$(ARGS); \
		if [ -d $$module ]; then \
			echo "Building module: $(ARGS)"; \
			$(MAKE) -C $$module; \
			AOC_LIB_PATH=$(LIB_DIR) $(ZIG) build run $(ZIG_BUILD_FLAGS) -- $(ARGS); \
		else \
			echo "Error: Module $(ARGS) does not exist."; \
			exit 1; \
		fi; \
	else \
		echo "Error: ARGS must be a two-digit module number (e.g., ARGS=01)"; \
		exit 1; \
	fi

clean:
	rm -rf $(BUILD_DIR) $(CACHE_DIR)
	@for module in $(MODULES); do \
		$(MAKE) -C $(SRC_DIR)/$$module clean; \
	done

# prevents make from treating args like targets
%:
	@:

