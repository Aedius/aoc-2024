use helper::{InputReader, Solver};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Sub};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "14".to_string(),
        result1: Some("354".to_string()),
        example2: Some("34".to_string()),
        result2: Some("1263".to_string()),
        kind: Default::default(),
    };

    solver.solve("day08");
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct Container {
    antennas: HashMap<char, Vec<Pos>>,
    width: isize,
    height: isize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.width = line.len() as isize;

        let chars: Vec<char> = line.chars().collect();
        for (y, c) in chars.iter().enumerate() {
            if c != &'.' {
                let entry = self.antennas.entry(*c).or_default();
                entry.push(Pos {
                    x: self.height,
                    y: y as isize,
                });
            }
        }

        self.height += 1;
    }

    fn star1(&self) -> String {
        let mut res = Vec::new();
        for (_char, positions) in self.antennas.clone() {
            for c in Self::get_couples(positions.clone()) {
                let mut computed = self.compute_star1(c.0, c.1);
                res.append(&mut computed);
            }
        }
        res.sort();
        res.dedup();
        res.len().to_string()
    }

    fn star2(&self) -> String {
        let mut res = Vec::new();
        for (_char, positions) in self.antennas.clone() {
            for c in Self::get_couples(positions.clone()) {
                let mut computed = self.compute_star2(c.0, c.1);
                res.append(&mut computed);
            }
        }
        res.sort();
        res.dedup();
        res.len().to_string()
    }
}

impl Container {
    fn get_couples(mut list: Vec<Pos>) -> Vec<(Pos, Pos)> {
        let mut result = Vec::new();
        while let Some(b) = list.pop() {
            for a in &list {
                result.push((*a, b))
            }
        }

        result
    }

    fn compute_star1(&self, a: Pos, b: Pos) -> Vec<Pos> {
        let mut res = Vec::new();
        let vec = b - a;

        let left = a - vec;
        if self.is_in_grid(left).is_some() {
            res.push(left)
        }
        let right = b + vec;
        if self.is_in_grid(right).is_some() {
            res.push(right)
        }

        res
    }

    fn compute_star2(&self, a: Pos, b: Pos) -> Vec<Pos> {
        let mut res = vec![a, b];
        let vec = b - a;

        let mut left = a;

        while let Some(is_in) = self.is_in_grid(left - vec) {
            res.push(is_in);
            left = is_in;
        }

        let mut right = b;
        while let Some(is_in) = self.is_in_grid(right + vec) {
            res.push(is_in);
            right = is_in;
        }

        res
    }

    fn is_in_grid(&self, p: Pos) -> Option<Pos> {
        if p.x < 0 || p.y < 0 {
            return None;
        }
        if p.x >= self.height || p.y >= self.width {
            return None;
        }

        Some(p)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x.cmp(&other.x) == Ordering::Equal {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}
