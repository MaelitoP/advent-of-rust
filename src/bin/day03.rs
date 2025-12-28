use advent_of_rust::aoc::input;
use advent_of_rust::aoc::parse::lines;

fn main() {
    let input = input::read_day(3);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    lines(input).map(max_two_digit_subsequence_value).sum()
}

fn part2(input: &str) -> u128 {
    lines(input)
        .map(|line| max_k_digit_subsequence_value(line, 12))
        .sum()
}

pub fn max_two_digit_subsequence_value(line: &str) -> i64 {
    let mut seen_any = false;
    let mut max_right: u8 = 0;
    let mut best: i64 = -1;

    for b in line.bytes().rev() {
        let d = match b {
            b'0'..=b'9' => b - b'0',
            _ => continue,
        };

        if seen_any {
            let candidate = (d as i64) * 10 + (max_right as i64);
            if candidate > best {
                best = candidate;
            }
            if d > max_right {
                max_right = d;
            }
        } else {
            seen_any = true;
            max_right = d;
        }
    }

    assert!(best >= 0, "line must contain at least 2 digits");
    best
}

pub fn max_k_digit_subsequence_value(line: &str, k: usize) -> u128 {
    assert!(k > 0, "k must be > 0");

    let n = line.bytes().filter(|b| (b'0'..=b'9').contains(b)).count();
    assert!(n >= k, "line must contain at least {} digits", k);

    let mut drop = n - k;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for b in line.bytes() {
        let d = match b {
            b'0'..=b'9' => b - b'0',
            _ => continue,
        };

        while drop > 0 && stack.last().is_some_and(|&last| last < d) {
            stack.pop();
            drop -= 1;
        }
        stack.push(d);
    }

    if drop > 0 {
        stack.truncate(stack.len() - drop);
    }

    let mut value: u128 = 0;
    for &d in stack.iter().take(k) {
        value = value * 10 + d as u128;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "#;

    #[test]
    fn part1_example() {
        assert_eq!(357, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(3121910778619, part2(EXAMPLE));
    }

    #[test]
    fn k_subsequence_examples() {
        assert_eq!(
            987654321111,
            max_k_digit_subsequence_value("987654321111111", 12)
        );
        assert_eq!(
            811111111119,
            max_k_digit_subsequence_value("811111111111119", 12)
        );
        assert_eq!(
            434234234278,
            max_k_digit_subsequence_value("234234234234278", 12)
        );
        assert_eq!(
            888911112111,
            max_k_digit_subsequence_value("818181911112111", 12)
        );
    }
}
