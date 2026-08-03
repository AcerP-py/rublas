PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin
LIBDIR := $(PREFIX)/lib
INCLUDEDIR := $(PREFIX)/include

TARGET_DIR := target/release
BINARY := $(TARGET_DIR)/rublas_info
LIBRARY := $(TARGET_DIR)/librublas
HEADERS := interfaces/c_cpp/rublas.h interfaces/c_cpp/cblas.h

INSTALL := install
LN := ln

.PHONY: all build install uninstall test clean

all: build

build:
	cargo build --release

install: build
	$(INSTALL) -d $(BINDIR)
	$(INSTALL) -m 755 $(BINARY) $(BINDIR)/
	$(INSTALL) -d $(LIBDIR)
	$(INSTALL) -m 755 $(LIBRARY).a $(LIBDIR)/
	$(INSTALL) -m 755 $(LIBRARY).so $(LIBDIR)/
	$(INSTALL) -d $(INCLUDEDIR)
	$(INSTALL) -m 644 $(HEADERS) $(INCLUDEDIR)/
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
	@cargo build --lib --features c_cpp,cblas > /dev/null
	@echo ""
	@$(MAKE) --no-print-directory -C tests/interfaces/c_cpp run PROJ_DIR="$(CURDIR)"

clean:
	@cargo clean
	@$(MAKE) --no-print-directory -C tests/interfaces/c_cpp clean
