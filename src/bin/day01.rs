use advent_of_rust::aoc::{input, parse::lines};

fn main() {
    let input = input::read_day(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut hits: i64 = 0;
    let mut value: i32 = 50;

    for (delta, _steps) in moves(input) {
        value = rotate(value, delta);
        if value == 0 {
            hits += 1;
        }
    }

    hits
}

fn part2(input: &str) -> i64 {
    let mut hits: i64 = 0;
    let mut value: i32 = 50;

    for (delta, steps) in moves(input) {
        hits += count_zero_landings(value, delta, steps);
        value = rotate(value, delta);
    }

    hits
}

fn moves(input: &str) -> impl Iterator<Item = (i32, i64)> + '_ {
    lines(input).map(|line| {
        let (dir, rest) = line.split_at(1);
        let n: i32 = rest.parse().expect("invalid number");

        let delta: i32 = match dir {
            "L" => -n,
            "R" => n,
            _ => panic!("unsupported direction: {dir}"),
        };

        let steps: i64 = i64::from(delta).abs();

        (delta, steps)
    })
}

fn rotate(curr: i32, delta: i32) -> i32 {
    (curr + delta).rem_euclid(100)
}

fn count_zero_landings(curr: i32, delta: i32, steps: i64) -> i64 {
    const MOD: i32 = 100;

    if steps == 0 {
        return 0;
    }

    // Right: curr + t ≡ 0 (mod 100) => t ≡ (100 - curr) mod 100
    // Left : curr - t ≡ 0 (mod 100) => t ≡ curr mod 100
    let first_hit: i64 = if delta > 0 {
        if curr == 0 { 100 } else { (MOD - curr) as i64 }
    } else {
        if curr == 0 { 100 } else { curr as i64 }
    };

    if steps < first_hit {
        0
    } else {
        1 + (steps - first_hit) / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "#;

    #[test]
    fn part1_example() {
        assert_eq!(3, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(6, part2(EXAMPLE));
    }

    #[test]
    fn rotate_basic() {
        assert_eq!(99, rotate(0, -1));
        assert_eq!(0, rotate(1, -1));
        assert_eq!(0, rotate(0, 100));
        assert_eq!(50, rotate(50, 0));
        assert_eq!(0, rotate(50, 50));
    }

    #[test]
    fn count_zero_landings_semantics() {
        assert_eq!(1, count_zero_landings(1, -1, 1));

        assert_eq!(0, count_zero_landings(0, 1, 1));
        assert_eq!(1, count_zero_landings(0, 100, 100));

        assert_eq!(0, count_zero_landings(0, 0, 0));

        assert_eq!(3, count_zero_landings(50, 250, 250));
    }
}
