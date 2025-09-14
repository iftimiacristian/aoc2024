#!/bin/bash
set -e

# --- Configuration ---
# The name of the runner crate
RUNNER_CRATE="runner"
# The name of the common crate
COMMON_CRATE="common"
# The directory for puzzle inputs
INPUTS_DIR="inputs"
# The root Cargo.toml file
ROOT_CARGO_TOML="Cargo.toml"
# The runner's Cargo.toml file
RUNNER_CARGO_TOML="$RUNNER_CRATE/Cargo.toml"


# --- Helper Functions ---
function print_usage() {
    echo "Usage: $0 <command> [day_number]"
    echo "Commands:"
    echo "  add <day_number>    - Adds a new day crate. e.g., add 5"
    echo "  remove <day_number> - Removes an existing day crate. e.g., remove 5"
    echo "  last                - Shows the last day added."
}

function get_last_day() {
    ls -d day?? 2>/dev/null | sort -n | tail -n 1
}

# --- Main Logic ---
COMMAND=$1
DAY_NUM=$2

if [[ -z "$COMMAND" ]]; then
    print_usage
    exit 1
fi

if [[ "$COMMAND" == "last" ]]; then
    LAST_DAY_DIR=$(get_last_day)
    if [[ -z "$LAST_DAY_DIR" ]]; then
        echo "No days have been added yet."
    else
        echo "Last day added: $LAST_DAY_DIR"
    fi
    exit 0
fi

if [[ -z "$DAY_NUM" ]]; then
    echo "Error: Missing day number."
    print_usage
    exit 1
fi

# Format day number to be two digits (e.g., 5 -> 05)
DAY_FMT=$(printf "%02d" "$DAY_NUM")
DAY_DIR="day$DAY_FMT"

# --- Add Command ---
if [[ "$COMMAND" == "add" ]]; then
    echo "--- Adding Day $DAY_FMT ---"

    if [ -d "$DAY_DIR" ]; then
        echo "Error: Directory '$DAY_DIR' already exists."
        exit 1
    fi

    LAST_DAY_DIR=$(get_last_day)
    if [[ -n "$LAST_DAY_DIR" ]]; then
        echo "Last day was: $LAST_DAY_DIR"
    fi

    echo "1. Creating crate '$DAY_DIR'..."
    cargo new "$DAY_DIR" --lib --vcs none

    echo "2. Adding '$COMMON_CRATE' dependency to '$DAY_DIR/Cargo.toml'..."
    echo "common = { path = \"../$COMMON_CRATE\" }" >> "$DAY_DIR/Cargo.toml"

    echo "3. Adding '$DAY_DIR' to workspace members in '$ROOT_CARGO_TOML'..."
    # This sed command inserts the new member just before the closing bracket ']'
    sed -i '/^]$/i \    "'$DAY_DIR'",' "$ROOT_CARGO_TOML"

    echo "4. Adding '$DAY_DIR' dependency to '$RUNNER_CARGO_TOML'..."
    # This sed command adds the dependency at the end of the [dependencies] section
    sed -i "/^\\[dependencies\\]/a $DAY_DIR = { path = \"../$DAY_DIR\" }" "$RUNNER_CARGO_TOML"

    echo "5. Creating input files in '$INPUTS_DIR/$DAY_DIR'..."
    mkdir -p "$INPUTS_DIR/$DAY_DIR"
    touch "$INPUTS_DIR/$DAY_DIR/example.txt"
    touch "$INPUTS_DIR/$DAY_DIR/full.txt"

    echo "--- Day $DAY_FMT added successfully! ---"

# --- Remove Command ---
elif [[ "$COMMAND" == "remove" ]]; then
    echo "--- Removing Day $DAY_FMT ---"

    if [ ! -d "$DAY_DIR" ]; then
        echo "Error: Directory '$DAY_DIR' does not exist."
        exit 1
    fi

    echo "1. Removing directory '$DAY_DIR'..."
    rm -rf "$DAY_DIR"

    echo "2. Removing '$DAY_DIR' from workspace members in '$ROOT_CARGO_TOML'..."
    # This sed command finds and deletes the line containing the day's entry
    sed -i "/$DAY_DIR/d" "$ROOT_CARGO_TOML"

    echo "3. Removing '$DAY_DIR' dependency from '$RUNNER_CARGO_TOML'..."
    sed -i "/$DAY_DIR =/d" "$RUNNER_CARGO_TOML"
    
    echo "4. Removing input files from '$INPUTS_DIR/$DAY_DIR'..."
    rm -rf "$INPUTS_DIR/$DAY_DIR"

    echo "--- Day $DAY_FMT removed successfully! ---"

else
    echo "Error: Unknown command '$COMMAND'"
    print_usage
    exit 1
fi