.DEFAULT_GOAL := help

# Show possible commands
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build (b)         - Build the project"
	@echo "  test (t)          - Run tests with all features"
	@echo "  clippy (lint)     - Run Clippy on the workspace"
	@echo "  fmt               - Format the project using nightly"

# Development group
.PHONY: build b
build b:
	cargo build

.PHONY: test t
test t:
	cargo test --all-features --all

.PHONY: clippy lint
clippy lint:
	cargo clippy --all-features

.PHONY: fmt
fmt:
	cargo +nightly fmt

