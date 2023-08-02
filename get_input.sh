#!/usr/bin/env bash
YEAR=$1
# Remove leading zeros from day
((DAY=10#$2))
SESSION=$(cat session.txt)

# If year in format YY, prefix it with '20'
if [[ $YEAR =~ ^[0-9]{2}$ ]]; then
  YEAR="20$YEAR"
fi

# Create data directory if it doesn't exist
mkdir -p "$YEAR/data"

# Download input
curl -b "session=$SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" \
  > "$YEAR/data/$(printf %02d "$DAY").input"
