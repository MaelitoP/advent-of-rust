use std::fs;

pub fn read_day(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
}

pub fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
}
