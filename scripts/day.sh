#!/bin/bash
set -e

# --- Configuration ---
RUNNER_CRATE="runner"
COMMON_CRATE="common"
INPUTS_DIR="inputs"
ROOT_CARGO_TOML="Cargo.toml"
RUNNER_CARGO_TOML="$RUNNER_CRATE/Cargo.toml"

# --- Helper Functions ---
function print_usage() {
    echo "Usage: $0 <command> [day_number]"
    echo "Commands:"
    echo "  add [day_number]    - Adds a new day crate. Auto-increments if no day number is given."
    echo "  remove <day_number> - Removes an existing day crate. e.g., remove 5"
    echo "  last                - Shows the last day added."
}

function get_last_day() {
    # Use find for robustness, and printf to print just the directory name.
    find . -maxdepth 1 -type d -name 'day??' -printf '%f\n' | sort -n | tail -n 1
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
        echo "Last day added: ${LAST_DAY_DIR}"
    fi
    exit 0
fi

# For remove command, day number is required
if [[ "$COMMAND" == "remove" && -z "$DAY_NUM" ]]; then
    echo "Error: Missing day number for 'remove' command."
    print_usage
    exit 1
fi

# --- Add Command ---
if [[ "$COMMAND" == "add" ]]; then
    # If day number is not provided, calculate the next one
    if [[ -z "$DAY_NUM" ]]; then
        echo "No day number specified, determining the next day..."
        LAST_DAY_DIR=$(get_last_day)
        if [[ -z "$LAST_DAY_DIR" ]]; then
            DAY_NUM=1
        else
            # Use shell parameter expansion for efficiency (SC2001)
            LAST_DAY_NUM=$((10#${LAST_DAY_DIR//day/}))
            DAY_NUM=$((LAST_DAY_NUM + 1))
        fi
        echo "New day will be: Day ${DAY_NUM}"
    fi

    DAY_FMT=$(printf "%02d" "$DAY_NUM")
    DAY_DIR="day${DAY_FMT}"

    echo "--- Adding Day ${DAY_FMT} ---"

    if [ -d "$DAY_DIR" ]; then
        echo "Error: Directory '${DAY_DIR}' already exists."
        exit 1
    fi

    echo "1. Creating crate '${DAY_DIR}' (and adding to workspace)..."
    cargo new "${DAY_DIR}" --lib --vcs none

    echo "2. Adding '${COMMON_CRATE}' dependency to '${DAY_DIR}/Cargo.toml' ..."
    echo "common = { path = \"../${COMMON_CRATE}\" }" >> "${DAY_DIR}/Cargo.toml"

    echo "3. Adding '${DAY_DIR}' dependency to '${RUNNER_CARGO_TOML}'..."
    sed -i "/^\\[dependencies\\]/a ${DAY_DIR} = { path = \"../${DAY_DIR}\" }" "$RUNNER_CARGO_TOML"

    echo "4. Creating input files for Part 1 and Part 2 in '${INPUTS_DIR}/${DAY_DIR}'..."
    mkdir -p "${INPUTS_DIR}/${DAY_DIR}"
    touch "${INPUTS_DIR}/${DAY_DIR}/example1.txt"
    touch "${INPUTS_DIR}/${DAY_DIR}/full1.txt"
    touch "${INPUTS_DIR}/${DAY_DIR}/example2.txt"
    touch "${INPUTS_DIR}/${DAY_DIR}/full2.txt"

    echo "--- Day ${DAY_FMT} added successfully! ---"

# --- Remove Command ---
elif [[ "$COMMAND" == "remove" ]]; then
    DAY_FMT=$(printf "%02d" "$DAY_NUM")
    DAY_DIR="day${DAY_FMT}"

    echo "--- Removing Day ${DAY_FMT} ---"

    if [ ! -d "$DAY_DIR" ]; then
        echo "Error: Directory '${DAY_DIR}' does not exist."
        exit 1
    fi

    echo "1. Removing '${DAY_DIR}' dependency from '${RUNNER_CARGO_TOML}'..."
    cargo remove "${DAY_DIR}" --manifest-path "${RUNNER_CARGO_TOML}"

    echo "2. Removing '${DAY_DIR}' from workspace members in '${ROOT_CARGO_TOML}'..."
    # A multi-step sed process to safely remove the member, even if it's on the same line as others.
    # 1. Remove the member string itself.
    sed -i 's/"'${DAY_DIR}'"//' "${ROOT_CARGO_TOML}"
    # 2. Clean up potential leftover comma issues.
    sed -i 's/,[[:space:]]*,/,/g' "${ROOT_CARGO_TOML}"
    sed -i 's/\\[[[:space:]]*,/\\[ /g' "${ROOT_CARGO_TOML}"
    sed -i 's/,[[:space:]]*\\]/ ]/g' "${ROOT_CARGO_TOML}"

    echo "3. Verifying and updating dependencies..."
    cargo check

    echo "4. Removing directory '${DAY_DIR}'..."
    rm -rf "${DAY_DIR:?}"
    
    echo "5. Removing input files from '${INPUTS_DIR}/${DAY_DIR}'..."
    rm -rf "${INPUTS_DIR:?}/${DAY_DIR:?}"

    echo "--- Day ${DAY_FMT} removed successfully! ---"

else
    echo "Error: Unknown command '${COMMAND}'"
    print_usage
    exit 1
fi