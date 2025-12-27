use advent_of_rust::aoc::input;

fn main() {
    let input = input::read_day(2);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    parse_intervals(input)
        .map(|(x, y)| repeated_twice_sum(x, y))
        .sum()
}

fn part2(input: &str) -> u64 {
    parse_intervals(input)
        .map(|(x, y)| repeated_block_numbers_sum(x, y))
        .sum()
}

/// Parses "a-b,c-d,..." into an iterator of (a,b).
fn parse_intervals(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(',').filter_map(|chunk| {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            return None;
        }

        let (a, b) = chunk
            .split_once('-')
            .unwrap_or_else(|| panic!("invalid interval (missing '-'): {chunk}"));

        let x: u64 = a
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("invalid number: {a}"));
        let y: u64 = b
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("invalid number: {b}"));

        Some((x, y))
    })
}

/// Sum of all n in [x, y] whose decimal representation is exactly two identical halves.
pub fn repeated_twice_sum(x: u64, y: u64) -> u64 {
    let mut sum = 0;

    for n in x..=y {
        if is_repeated_twice(n) {
            sum += n;
        }
    }

    sum
}

fn is_repeated_twice(n: u64) -> bool {
    // Handle 0 safely (ilog10 undefined for 0)
    let digits: u32 = if n == 0 { 1 } else { n.ilog10() + 1 };

    if digits % 2 != 0 {
        return false;
    }

    let half = digits / 2;
    let pow = 10_u64.pow(half);

    let left = n / pow;
    let right = n % pow;

    left == right
}

/// Returns true if `n` is made of a digit block repeated at least twice.
///
/// # Examples:
/// * 1010 ("10" * 2)
/// * 1111111 ("1" * 7)
/// * 123123123 ("123" * 3)
pub fn is_repeated_block_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len < 2 {
        return false;
    }

    // block_len must divide len, and repetitions >= 2 => block_len <= len/2
    for block_len in 1..=len / 2 {
        if len % block_len != 0 {
            continue;
        }
        let reps = len / block_len; // always >= 2 here

        let block = &bytes[0..block_len];
        let mut ok = true;

        for i in 1..reps {
            let start = i * block_len;
            let end = start + block_len;
            if &bytes[start..end] != block {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

/// Sum of all numbers in [x, y] whose decimal representation is a block repeated >= 2 times.
pub fn repeated_block_numbers_sum(x: u64, y: u64) -> u64 {
    let mut sum = 0;

    for n in x..=y {
        if is_repeated_block_at_least_twice(n) {
            sum += n;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part1_example() {
        assert_eq!(1227775554, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(4174379265, part2(EXAMPLE));
    }
}
