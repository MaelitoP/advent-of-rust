#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub r: i32,
    pub c: i32,
}

impl Pos {
    pub const fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    pub fn neighbors4(self) -> [Pos; 4] {
        [
            Pos::new(self.r - 1, self.c),
            Pos::new(self.r + 1, self.c),
            Pos::new(self.r, self.c - 1),
            Pos::new(self.r, self.c + 1),
        ]
    }

    pub fn neighbors8(self) -> [Pos; 8] {
        [
            Pos::new(self.r - 1, self.c - 1),
            Pos::new(self.r - 1, self.c),
            Pos::new(self.r - 1, self.c + 1),
            Pos::new(self.r, self.c - 1),
            Pos::new(self.r, self.c + 1),
            Pos::new(self.r + 1, self.c - 1),
            Pos::new(self.r + 1, self.c),
            Pos::new(self.r + 1, self.c + 1),
        ]
    }
}

pub fn parse_char_grid(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

pub fn in_bounds<T>(g: &[Vec<T>], p: Pos) -> bool {
    if p.r < 0 || p.c < 0 {
        return false;
    }
    let r = p.r as usize;
    if r >= g.len() {
        return false;
    }
    let c = p.c as usize;
    c < g[r].len()
}
