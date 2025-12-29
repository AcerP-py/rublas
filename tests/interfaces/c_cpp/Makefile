CC := gcc

CFLAGS := -I$(PROJ_DIR)/interfaces/c_cpp
LDFLAGS := -L$(PROJ_DIR)/target/debug -lrublas

SRC := $(wildcard *.c)
BIN := $(patsubst %.c,%,$(SRC))

.PHONY: all clean run

all: run

%: %.c
	@$(CC) $< -o $@ $(CFLAGS) $(LDFLAGS)

run: $(BIN)
	@for exe in $(BIN); do \
		echo "    \033[1;32mRunning\033[0m $${exe} interface tests\n"; \
		LD_LIBRARY_PATH=$(PROJ_DIR)/target/debug $(CURDIR)/$${exe}; \
		echo ""; \
	done

clean:
	@rm -f $(BIN)
