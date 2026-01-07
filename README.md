# Advent of Code Solutions in Rust

This repository contains my take on solving [Advent of Code](https://adventofcode.com/) challenges.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- [just](https://github.com/casey/just) command runner (optional but recommended)

### CLI Usage

The CLI tool provides commands to run, list, and download puzzle inputs.

#### Using `just` (Recommended)

```bash
# Run all solutions from all years
just run

# Run all solutions from a specific year
just run -y 2024

# Run a specific day
just run -y 2024 -d 1

# List all available solutions
just list

# List solutions from a specific year
just list -y 2025

# Run linting
just lint
```

#### Using `cargo` directly

```bash
# Run all solutions
cargo run --release --bin cli -- run

# Run solutions for a specific year
cargo run --release --bin cli -- run --year 2024

# Run a specific day
cargo run --release --bin cli -- run --year 2024 --day 1

# List all available solutions
cargo run --release --bin cli -- list

# Download puzzle input (requires AOC_SESSION in .env)
cargo run --release --bin cli -- get-input --year 2024 --day 1
```

### Environment Setup

To download puzzle inputs, create a `.env` file in the root directory:

```
AOC_SESSION=your_session_cookie_here
```

You can find your session cookie in your browser's developer tools after logging
into [adventofcode.com](https://adventofcode.com/).

## Solutions Progress

### Advent of Code 2024

| Day       | 1  | 2  | 3  | 4  | 5  |
|-----------|----|----|----|----|----|
| **1-5**   | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ |
| **6-10**  | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ |
| **11-15** | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ |
| **16-20** |    |    |    |    |    |
| **21-25** |    |    |    |    |    |

**Total: 30/50 ⭐**

### Advent of Code 2025

| Day       | 1  | 2  | 3  | 4  | 5  |
|-----------|----|----|----|----|----|
| **1-5**   | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ |
| **6-10**  | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐ |
| **11-12** | ⭐⭐ | ⭐⭐ |    |    |    |

**Total: 24/24 ⭐**

## Project Structure

```
.
├── aoc24/          # Advent of Code 2024 solutions
├── aoc25/          # Advent of Code 2025 solutions
├── cli/            # CLI tool for running solutions
├── utils/          # Shared functions and helper classes
└── justfile        # Command shortcuts
```

## License

This project is open source and available under the MIT License.

