# Puzzles unlock at midnight EST (UTC-5), so "today" follows New York time.
aoc_year := `TZ=America/New_York date +%Y`
aoc_day := `TZ=America/New_York date +%-d`

# Keep these in sync with the revs in prek.toml.
tombi := "uvx tombi@1.11.3"
typos := "uvx typos@1.52.2"

# List available recipes.
default:
    @just --list

# Run cargo fmt.
fmt *ARGS:
    cargo fmt --all {{ARGS}}

# Run cargo check.
check *ARGS:
    cargo check --workspace --all-targets {{ARGS}}

# Run cargo clippy.
clippy *ARGS:
    cargo clippy --workspace --all-targets {{ARGS}}

# Apply clippy, rustfmt and tombi fixes in place.
fix: (clippy "--fix" "--allow-dirty") fmt
    {{tombi}} format

# Run code linting tools without modifying files.
lint: (fmt "--check")
    CARGO_BUILD_WARNINGS=deny just clippy --keep-going
    {{tombi}} format --check
    {{tombi}} lint
    {{typos}}

# Run the unit tests.
test:
    cargo test --release --workspace --all-targets

# Run the answer tests.
test-answers:
    cargo test --release --workspace --all-targets --all-features --no-fail-fast

# Run all prek hooks on all files.
prek:
    prek run --all-files

# Run a day's solution (defaults to today's puzzle).
run-day day=aoc_day year=aoc_year *ARGS:
    cargo run --release -p y{{year}}-d{{ shell('printf %02d "${1#0}"', day) }} {{ARGS}}

# Run a day's unit tests (defaults to today's puzzle).
test-day day=aoc_day year=aoc_year *ARGS:
    cargo test --release -p y{{year}}-d{{ shell('printf %02d "${1#0}"', day) }} {{ARGS}}

# Run a day's answer tests (defaults to today's puzzle).
test-day-answers day=aoc_day year=aoc_year *ARGS:
    cargo test --release -p y{{year}}-d{{ shell('printf %02d "${1#0}"', day) }} --all-features {{ARGS}}

# Create a new day from the template (defaults to today's puzzle).
new-day day=aoc_day year=aoc_year:
    #!/usr/bin/env bash
    set -euo pipefail
    day=$((10#{{day}}))
    if (( day < 1 || day > 25 )); then
        echo "Day must be between 1 and 25, got {{day}}." >&2
        exit 1
    fi
    padded=$(printf %02d "$day")
    dir="{{year}}/day$padded"
    if [[ -e "$dir" ]]; then
        echo "$dir already exists." >&2
        exit 1
    fi
    mkdir -p "$dir/src"
    substitute() {
        sed -e "s/__YEAR__/{{year}}/g" -e "s/__DAY_PADDED__/$padded/g" -e "s/__DAY__/$day/g" "$1" > "$2"
    }
    substitute template/Cargo.toml.tmpl "$dir/Cargo.toml"
    substitute template/main.rs.tmpl "$dir/src/main.rs"
    echo "Created $dir. Run 'just test-day $day {{year}}' to fetch the input and examples."
