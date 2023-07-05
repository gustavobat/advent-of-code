#!/usr/bin/env bash
YEAR=$1
DAY=$2
SESSION=$(cat session.txt)
curl -b "session=$SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" \
  > "$YEAR/data/$(printf %02d "$DAY").input"
