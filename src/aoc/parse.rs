pub fn lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().map(str::trim).filter(|l| !l.is_empty())
}

pub fn ints_i64(s: &str) -> Vec<i64> {
    lines(s).map(|l| l.parse::<i64>().unwrap()).collect()
}

pub fn csv_i64(line: &str) -> Vec<i64> {
    line.trim()
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

/// Split input into blocks separated by blank lines.
pub fn blocks(s: &str) -> Vec<&str> {
    s.split("\n\n")
        .map(str::trim)
        .filter(|b| !b.is_empty())
        .collect()
}
