#!/usr/bin/env make

BINARY_NAME=doc-checker
INSTALL_PATH=/usr/local/bin/$(BINARY_NAME)

.PHONY: build test clean install uninstall run help

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the binary"
	@echo "  test       - Run tests"
	@echo "  clean      - Remove built binaries"
	@echo "  install    - Install binary to $(INSTALL_PATH)"
	@echo "  uninstall  - Remove installed binary"
	@echo "  run        - Run with example arguments"
	@echo "  help       - Show this help"

# Build the binary
build:
	go build -o $(BINARY_NAME) .

# Run tests
test:
	go test -v ./...

# Clean build artifacts
clean:
	rm -f $(BINARY_NAME)
	go clean

# Install to system
install: build
	sudo cp $(BINARY_NAME) $(INSTALL_PATH)
	@echo "Installed $(BINARY_NAME) to $(INSTALL_PATH)"

# Uninstall from system
uninstall:
	sudo rm -f $(INSTALL_PATH)
	@echo "Removed $(INSTALL_PATH)"

# Run with example arguments (from project root)
run: build
	cd ../.. && FORCE_COLOR=1 ./tools/doc-checker/$(BINARY_NAME) -v

# Run with JSON output
run-json: build
	cd ../.. && ./tools/doc-checker/$(BINARY_NAME) -o json

# Run on specific file
run-readme: build
	cd ../.. && FORCE_COLOR=1 ./tools/doc-checker/$(BINARY_NAME) -f README.md -v

# Development helpers
dev-test: build
	cd ../.. && FORCE_COLOR=1 ./tools/doc-checker/$(BINARY_NAME) -f README.md -v --quick

fmt:
	go fmt ./...

vet:
	go vet ./...

# Full quality check
check: fmt vet test
	@echo "All checks passed!"
