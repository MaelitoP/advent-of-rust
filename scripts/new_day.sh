#!/usr/bin/env bash
set -euo pipefail

DAY="${1:-}"

if [[ -z "$DAY" ]]; then
  echo "Usage: scripts/new_day.sh <day-number>"
  exit 1
fi

if ! [[ "$DAY" =~ ^[0-9]+$ ]]; then
  echo "Day must be a positive integer, got: '$DAY'"
  exit 1
fi

DAY_PADDED=$(printf "%02d" "$DAY")
BIN_PATH="src/bin/day${DAY_PADDED}.rs"
INPUT_DIR="inputs"
INPUT_PATH="${INPUT_DIR}/day${DAY_PADDED}.txt"

if [[ -e "$BIN_PATH" ]]; then
  echo "File $BIN_PATH already exists, aborting."
  exit 1
fi

mkdir -p "src/bin"
mkdir -p "$INPUT_DIR"

cat > "$BIN_PATH" <<EOF
use advent_of_rust::aoc::input;

fn main() {
    let input = input::read_day(${DAY});
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    // TODO: implement part 1
    0
}

fn part2(input: &str) -> i64 {
    // TODO: implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part1_example() {
        assert_eq!(0, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
EOF

echo "Created ${BIN_PATH}"

# Download input if AOC_SESSION is set
if [[ -z "${AOC_SESSION:-}" ]]; then
  echo "Environment variable AOC_SESSION is not set."
  echo "Skipping input download. You can download manually with:"
  echo "  curl https://adventofcode.com/2025/day/${DAY}/input --cookie \"session=<your_session>\" -o ${INPUT_PATH}"
else
  echo "Downloading input for day ${DAY}..."
  curl "https://adventofcode.com/2025/day/${DAY}/input" \
    --cookie "session=${AOC_SESSION}" \
    --silent --show-error --fail \
    -o "${INPUT_PATH}"
  echo "Saved input to ${INPUT_PATH}"
fi
