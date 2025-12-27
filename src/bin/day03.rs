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

fn part2(_input: &str) -> i64 {
    // TODO: implement part 2
    0
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
        assert_eq!(0, part2(EXAMPLE));
    }
}
