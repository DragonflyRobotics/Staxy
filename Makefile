# Makefile

# The name of the executable
EXEC = Staxy 

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the project
run: build
	./target/release/$(EXEC)

# Clean the project
clean:
	cargo clean

.PHONY: all build run clean
