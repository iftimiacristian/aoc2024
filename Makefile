# Makefile for Advent of Code 2024

# --- Variables ---
# If DAY is not passed, it will be empty. Used for new-day auto-increment.
DAY ?=
# Pad the day number to two digits (e.g., 1 -> 01). Only works if DAY is set.
DAY_FMT = $(shell printf "%02d" $(DAY))

# --- Cargo Commands ---
CARGO_RUN   = cargo run
CARGO_BUILD = cargo build --workspace
CARGO_TEST  = cargo test --workspace
CARGO_CHECK = cargo check --workspace
CARGO_CLEAN = cargo clean

# --- Phony Targets ---
.PHONY: help all build check fmt run test clean new-day remove-day last-day

# --- Main Targets ---

## Show this help message
help:
	@echo "Usage: make <target> [DAY=N]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %%-18s %%s\n", $$1, $$2}'

## Build all crates in the workspace
build:
	@echo "Building workspace..."
	@$(CARGO_BUILD)

## Check: clippy, fmt, tests
check:
	@echo "🔍 Running clippy (all targets)..."
	@cargo clippy --workspace --all-targets --all-features -- -D warnings
	@echo "📐 Checking code formatting..."
	@cargo fmt --all -- --check
	@echo "🧪 Running all tests..."
	@cargo test --workspace --all-features
	@echo "✅ All checks passed!"

## Fix formatting automatically
fmt:
	@echo "🔧 Fixing code formatting..."
	@cargo fmt --all

## Run the solution for a specific day (e.g., make run DAY=5)
run:
	@echo "Running solution..."
ifeq ($(DAY),)
	@$(CARGO_RUN) --bin runner
else
	@echo "Running solution for Day $(DAY_FMT)..."
	@$(CARGO_RUN) --bin runner -- $(DAY_FMT)
endif

## Run tests for a specific day (e.g., make test-day DAY=1)
test-day:
	@echo "Running tests for Day $(DAY_FMT)..."
	@$(CARGO_TEST) -p day$(DAY_FMT)

## Run all tests in the workspace
test:
	@echo "Running all tests in workspace..."
	@$(CARGO_TEST)

## Clean the target directory
clean:
	@echo "Cleaning workspace..."
	@$(CARGO_CLEAN)

## Create a new day crate. Auto-increments if DAY is not set (e.g., make new-day, or make new-day DAY=5)
new-day:
	@./scripts/day.sh add $(DAY)

## Remove an existing day crate (e.g., make remove-day DAY=5)
remove-day:
	@./scripts/day.sh remove $(DAY)

## Show the last day that was added
last-day:
	@./scripts/day.sh last