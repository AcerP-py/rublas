PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin
LIBDIR := $(PREFIX)/lib
INCLUDEDIR := $(PREFIX)/include

TARGET_DIR := target/release
BINARY := $(TARGET_DIR)/rublas_info
LIBRARY := $(TARGET_DIR)/librublas
HEADERDIR := interfaces/c_cpp/rublas.h interfaces/c_cpp/cblas.h

.PHONY: all build install uninstall test clean

all: build

build:
	cargo build --release

install: build
	install -d $(BINDIR)
	install -m 755 $(BINARY) $(BINDIR)
	install -d $(LIBDIR)
	install -m 755 $(LIBRARY).so $(LIBDIR)
	install -m 755 $(LIBRARY).a $(LIBDIR)
	install -d $(INCLUDEDIR)
	install -m 644 $(HEADER) $(INCLUDEDIR)
	@echo "Installed rublas_info to $(BINDIR)"
	@echo "Installed librublas.so and librublas.a to $(LIBDIR)"
	@echo "Installed rublas.h and cblas.h to $(INCLUDEDIR)"

uninstall:
	rm -f $(BINDIR)
	rm -f $(LIBDIR)
	rm -f $(INCLUDEDIR)
	@echo "Uninstalled all rublas components"

test:
	cargo test

clean:
	cargo clean
