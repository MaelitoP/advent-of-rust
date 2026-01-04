use advent_of_rust::aoc::grid::{in_bounds, parse_char_grid, Pos};
use advent_of_rust::aoc::input;
use std::collections::VecDeque;

fn main() {
    let input = input::read_day(4);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let grid = parse_char_grid(input);
    count_accessible_rolls(&grid)
}

fn part2(input: &str) -> usize {
    let grid = parse_char_grid(input);
    count_removable_rolls(&grid)
}

fn count_accessible_rolls(grid: &[Vec<char>]) -> usize {
    all_positions(grid)
        .filter(|&p| cell(grid, p) == Some('@'))
        .filter(|&p| adjacent_roll_count(grid, p) < 4)
        .count()
}

fn adjacent_roll_count(grid: &[Vec<char>], p: Pos) -> usize {
    p.neighbors8()
        .into_iter()
        .filter(|&n| cell(grid, n) == Some('@'))
        .count()
}

fn count_removable_rolls(grid: &[Vec<char>]) -> usize {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut present: Vec<bool> = Vec::with_capacity(h * w);
    for r in 0..h {
        debug_assert_eq!(grid[r].len(), w, "grid must be rectangular");
        for c in 0..w {
            present.push(grid[r][c] == '@');
        }
    }

    // Initial neighbor counts for each cell (only meaningful for present rolls)
    let mut deg: Vec<u8> = vec![0; h * w];
    for r in 0..h {
        for c in 0..w {
            let p = Pos::new(r as i32, c as i32);
            let i = idx(r, c, w);
            if !present[i] {
                continue;
            }

            let mut count: u8 = 0;
            for n in p.neighbors8() {
                if !in_bounds(grid, n) {
                    continue;
                }
                let nr = n.r as usize;
                let nc = n.c as usize;
                if present[idx(nr, nc, w)] {
                    count += 1;
                }
            }
            deg[i] = count;
        }
    }

    // Queue all currently accessible rolls
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..present.len() {
        if present[i] && deg[i] < 4 {
            q.push_back(i);
        }
    }

    // Repeatedly remove, updating neighbor degrees
    let mut removed = 0usize;
    while let Some(i) = q.pop_front() {
        if !present[i] {
            continue; // might have been removed already or enqueued multiple times
        }
        if deg[i] >= 4 {
            continue; // no longer accessible
        }

        present[i] = false;
        removed += 1;

        let (r, c) = (i / w, i % w);
        let p = Pos::new(r as i32, c as i32);

        // Decrement degree of each adjacent present roll
        // If it becomes accessible, enqueue
        for n in p.neighbors8() {
            if !in_bounds(grid, n) {
                continue;
            }
            let nr = n.r as usize;
            let nc = n.c as usize;
            let j = idx(nr, nc, w);

            if !present[j] {
                continue;
            }

            // This neighbor just lost one adjacent roll
            deg[j] = deg[j].saturating_sub(1);

            if deg[j] < 4 {
                q.push_back(j);
            }
        }
    }

    removed
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

#[inline]
fn idx(r: usize, c: usize, w: usize) -> usize {
    r * w + c
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
        assert_eq!(43, part2(EXAMPLE));
    }
}
