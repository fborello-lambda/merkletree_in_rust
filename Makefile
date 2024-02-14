# Default target
TARGET ?= debug

# Phony targets
.PHONY: all build run test clean help

# Default target
all: build

# Build target
build:
	cargo build --$(TARGET)

# Run target
run: build
	cargo run --$(TARGET)

# Test target
test:
	cargo test --workspace --all-targets --all-features

# Clean target
clean:
	cargo clean

# Help target
help:
	@echo "Usage: make TARGET=[target]"
	@echo "With target == debug || release"
	@echo ""
	@echo "Targets:"
	@echo "  all    - Build the project (default)"
	@echo "  build  - Build the project"
	@echo "  run    - Run the project"
	@echo "  test   - Run tests"
	@echo "  clean  - Clean the project"
	@echo "  help   - Display this help message"

