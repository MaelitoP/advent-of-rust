use advent_of_rust::aoc::input;

#[derive(Debug)]
pub struct Database {
    pub ranges: Vec<(i64, i64)>,
    pub ids: Vec<i64>,
}

fn main() {
    let input = input::read_day(5);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let db = parse_database(input).unwrap();
    let merged = merge_ranges(db.ranges);

    db.ids
        .into_iter()
        .filter(|&id| is_fresh(&merged, id))
        .count()
}

fn part2(_input: &str) -> i64 {
    0
}

pub fn parse_database(input: &str) -> Result<Database, &'static str> {
    let s = input.trim();
    let mut parts = s.splitn(2, "\n\n");

    let ranges_part = parts.next().ok_or("missing first block")?.trim();
    let ids_part = parts
        .next()
        .ok_or("missing second block (no blank line separator)")?
        .trim();

    if ranges_part.is_empty() {
        return Err("first block is empty");
    }
    if ids_part.is_empty() {
        return Err("second block is empty");
    }

    let ranges = ranges_part
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').ok_or("invalid range format")?;
            let a: i64 = a.trim().parse().map_err(|_| "invalid number")?;
            let b: i64 = b.trim().parse().map_err(|_| "invalid number")?;
            Ok((a, b))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let ids = ids_part
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| line.parse::<i64>().map_err(|_| "invalid id"))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Database { ranges, ids })
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    for (a, b) in &mut ranges {
        if *a > *b {
            std::mem::swap(a, b);
        }
    }

    ranges.sort_unstable_by_key(|&(a, _)| a);

    let mut merged: Vec<(i64, i64)> = Vec::with_capacity(ranges.len());
    for (start, end) in ranges {
        match merged.last_mut() {
            None => merged.push((start, end)),
            Some((_ms, me)) => {
                if start <= *me + 1 {
                    *me = (*me).max(end);
                } else {
                    merged.push((start, end));
                }
            }
        }
    }
    merged
}

fn is_fresh(merged: &[(i64, i64)], id: i64) -> bool {
    let idx = match merged.binary_search_by(|&(start, _)| start.cmp(&id)) {
        Ok(i) => i,
        Err(0) => return false,
        Err(i) => i - 1,
    };
    let (_start, end) = merged[idx];
    id <= end
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "#;

    #[test]
    fn part1_example() {
        assert_eq!(3, part1(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
