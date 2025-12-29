PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin
LIBDIR := $(PREFIX)/lib
INCLUDEDIR := $(PREFIX)/include

TARGET_DIR := target/release
BINARY := $(TARGET_DIR)/rublas_info
LIBRARY := $(TARGET_DIR)/librublas
HEADERS := interfaces/c_cpp/rublas.h interfaces/c_cpp/cblas.h

.PHONY: all build install uninstall test clean

all: build

build:
	cargo build --release

install: build
	install -d $(BINDIR)
	install -m 755 $(BINARY) $(BINDIR)/
	install -d $(LIBDIR)
	install -m 755 $(LIBRARY).so $(LIBRARY).a $(LIBDIR)/
	install -d $(INCLUDEDIR)
	install -m 644 $(HEADERS) $(INCLUDEDIR)/
	@echo "Installed rublas_info to $(BINDIR)"
	@echo "Installed librublas.so and librublas.a to $(LIBDIR)"
	@echo "Installed rublas.h and cblas.h to $(INCLUDEDIR)"

uninstall:
	rm -rf $(PREFIX)
	@echo "Uninstalled all rublas components"

test:
	@cargo test
# C_CPP API tests
	@echo "\033[1;32mC_CPP API tests\033[0m\n"
	@cargo build --lib > /dev/null
	@echo ""
	@$(MAKE) --no-print-directory -C tests/interfaces/c_cpp run PROJ_DIR="$(CURDIR)"

clean:
	@cargo clean
	@$(MAKE) --no-print-directory -C tests/interfaces/c_cpp clean
