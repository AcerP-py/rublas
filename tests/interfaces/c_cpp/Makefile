CC := gcc

CFLAGS := -I$(PROJ_DIR)/interfaces/c_cpp
LDFLAGS := -L$(PROJ_DIR)/target/debug -lrublas

SRC := $(wildcard *.c)
BIN_DIR := $(PROJ_DIR)/target/debug/c_cpp
BIN := $(patsubst %.c,$(BIN_DIR)/%,$(SRC))

.PHONY: all clean run

all: run

$(BIN_DIR):
	@mkdir -p $(BIN_DIR)

$(BIN_DIR)/%: %.c | $(BIN_DIR)
	@$(CC) $< -o $@ $(CFLAGS) $(LDFLAGS)

run: $(BIN)
	@for exe in $(BIN); do \
	    name=$$(basename "$$exe"); \
		echo "    \033[1;32mRunning\033[0m $${name} interface tests\n"; \
		LD_LIBRARY_PATH=$(PROJ_DIR)/target/debug $${exe}; \
		echo ""; \
	done

clean:
	@rm -rf $(BIN_DIR)
