use helper::{InputReader, Solver};
use std::cmp::Ordering;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "36".to_string(),
        result1: Some("694".to_string()),
        example2: Some("81".to_string()),
        result2: Some("1497".to_string()),
        kind: Default::default(),
    };

    solver.solve("day10");
}

#[derive(Debug, Default)]
struct Container {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        self.height = self.map.len();
        self.width = self.map[0].len();
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        self.map.push(
            line.chars()
                .map(|c| c.to_digit(10).or(Some(100)).expect("oups"))
                .collect(),
        )
    }

    fn star1(&self) -> String {
        let mut res = 0;

        for start in self.path_start() {
            res += self.get_trailhead_star_1(&start, 0).len();
        }

        res.to_string()
    }

    fn star2(&self) -> String {
        let mut res = 0;

        for start in self.path_start() {
            res += self.get_trailhead_star_2(&start, 0);
        }

        res.to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialOrd<Self> for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.y.cmp(&other.y),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl Container {
    fn path_start(&self) -> Vec<Pos> {
        let mut res = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if self.map[x][y] == 0 {
                    res.push(Pos { x, y });
                }
            }
        }

        res
    }

    fn get_next(&self, pos: &Pos, value: u32) -> Vec<Pos> {
        let mut res = Vec::new();

        if pos.x > 0 && self.map[pos.x - 1][pos.y] == value + 1 {
            res.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
        }

        if pos.y > 0 && self.map[pos.x][pos.y - 1] == value + 1 {
            res.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
        }

        if pos.x < self.width - 1 && self.map[pos.x + 1][pos.y] == value + 1 {
            res.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
        }

        if pos.y < self.height - 1 && self.map[pos.x][pos.y + 1] == value + 1 {
            res.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
        }

        res
    }

    fn get_trailhead_star_1(&self, pos: &Pos, value: u32) -> Vec<Pos> {
        if value == 9 {
            return vec![pos.clone()];
        }

        let nexts = self.get_next(pos, value);

        let mut res = Vec::new();

        for next in &nexts {
            let mut local = self.get_trailhead_star_1(next, value + 1);
            res.append(&mut local);
        }

        res.sort();
        res.dedup();

        res
    }
    fn get_trailhead_star_2(&self, pos: &Pos, value: u32) -> usize {
        if value == 9 {
            return 1;
        }

        let nexts = self.get_next(pos, value);

        let mut res = 0;

        for next in &nexts {
            res += self.get_trailhead_star_2(next, value + 1);
        }

        res
    }
}
