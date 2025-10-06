PREFIX ?= /usr/local
LIBDIR := $(PREFIX)/lib
INCLUDEDIR := $(PREFIX)/include

# Paths to built artifacts
TARGET_DIR := target/release
LIBRARY := $(TARGET_DIR)/librublas
HEADER := include/rublas.h

.PHONY: all build install uninstall test clean

# Default target
all: build

# Build the Rust shared library
build:
	cargo build --release

# Install the shared library and header
install: build
	install -d $(LIBDIR)
	install -m 755 $(LIBRARY).so $(LIBDIR)
	install -m 755 $(LIBRARY).a $(LIBDIR)
	install -d $(INCLUDEDIR)
	install -m 644 $(HEADER) $(INCLUDEDIR)
	@echo "Installed librublas.so and librublas.a to $(LIBDIR)"
	@echo "Installed rublas.h to $(INCLUDEDIR)"

# Uninstall the library and header
uninstall:
	rm -f $(LIBDIR)/librublas.so
	rm -f $(LIBDIR)/librublas.a
	rm -f $(INCLUDEDIR)/rublas.h
	@echo "Uninstalled librublas.so, librublas.a and rublas.h"

test:
	@cargo test
	@cargo build --release --quiet
	@cd tests; $(MAKE) --no-print-directory -f Makefile.c_tests; cd ..

# Clean build artifacts
clean:
	cargo clean
	@cd tests; $(MAKE) --no-print-directory -f Makefile.c_tests clean; cd ..
