# Justfile for advent-of-code
# Simplified shortcuts for running the CLI and linting.
# Examples:
#   just run --year 2025
#   just run --year 2025 --day 5
#   just list --year 2025
#   just run-debug --year 2025
#   just lint

# Run solvers in release mode (forwards optinal filters).
run *args:
    cargo run --release --bin cli -- run {{ args }}

# Run solvers in debug mode (forwards optional filters).
run-debug *args:
    cargo run --bin cli -- run {{ args }}

# List solvers available (forwards optional filters).
list *args:
    cargo run --release --bin cli -- list {{ args }}

# Lint: run clippy for all targets, then format with nightly rustfmt.
lint:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo +nightly fmt --all
