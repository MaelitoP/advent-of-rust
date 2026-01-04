use advent_of_rust::aoc::grid::{in_bounds, parse_char_grid, Pos};
use advent_of_rust::aoc::input;

fn main() {
    let input = input::read_day(4);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let grid = parse_char_grid(input);

    count_accessible_rolls(&grid)
}

fn part2(_input: &str) -> i64 {
    // TODO: implement part 2
    0
}

fn count_accessible_rolls(grid: &[Vec<char>]) -> usize {
    all_positions(grid)
        .filter(|&p| is_roll(grid, p))
        .filter(|&p| adjacent_roll_count(grid, p) < 4)
        .count()
}

fn is_roll(grid: &[Vec<char>], p: Pos) -> bool {
    cell(grid, p) == Some('@')
}

fn adjacent_roll_count(grid: &[Vec<char>], p: Pos) -> usize {
    p.neighbors8()
        .into_iter()
        .filter(|&n| cell(grid, n) == Some('@'))
        .count()
}

fn cell(grid: &[Vec<char>], p: Pos) -> Option<char> {
    if !in_bounds(grid, p) {
        return None;
    }
    Some(grid[p.r as usize][p.c as usize])
}

fn all_positions<T>(g: &[Vec<T>]) -> impl Iterator<Item = Pos> + '_ {
    g.iter().enumerate().flat_map(|(r, row)| {
        let r = r as i32;
        (0..row.len() as i32).map(move |c| Pos::new(r, c))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "#;

    #[test]
    fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
