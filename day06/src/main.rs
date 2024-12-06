use helper::{InputReader, Solver};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "41".to_string(),
        result1: Some("5453".to_string()),
        example2: Some("6".to_string()),
        result2: Some("2188".to_string()),
        kind: Default::default(),
    };

    solver.solve("day06");
}

#[derive(Debug, Default, Clone)]
struct Container {
    map: Vec<Vec<Tile>>,
    start: Gard,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
}
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    Est,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::Est,
            Direction::Est => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Gard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        let mut row = Vec::new();

        for c in line.chars() {
            match c {
                '.' => row.push(Tile::Empty),
                '#' => row.push(Tile::Obstacle),
                '^' => {
                    self.start = Gard {
                        direction: Direction::North,
                        x: self.map.len(),
                        y: row.len(),
                    };
                    row.push(Tile::Empty);
                }
                _ => {
                    panic!("nothing more");
                }
            }
        }

        self.map.push(row);
    }

    fn star1(&self) -> String {
        let path = self.generate_path().unwrap();

        path.len().to_string()
    }

    fn star2(&self) -> String {
        let mut possible = self.generate_path().unwrap();

        let tr = (self.start.x, self.start.y);
        possible.remove(&tr);

        let mut res = 0;

        for spot in possible {
            let mut test = self.clone();
            test.map[spot.0][spot.1] = Tile::Obstacle;

            if test.generate_path().is_none() {
                res += 1;
            }
        }

        res.to_string()
    }
}

impl Container {
    fn screen(&self, path: &HashSet<(usize, usize)>, gard: &Gard) {
        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                let s = if x == gard.x && y == gard.y {
                    match gard.direction {
                        Direction::North => "^",
                        Direction::Est => ">",
                        Direction::South => "<",
                        Direction::West => "v",
                    }
                } else if path.contains(&(x, y)) {
                    "X"
                } else {
                    match self.map[x][y] {
                        Tile::Empty => ".",
                        Tile::Obstacle => "#",
                    }
                };
                print!("{s}");
            }
            println!()
        }
    }

    fn generate_path(&self) -> Option<HashSet<(usize, usize)>> {
        let mut gard = self.start.clone();
        let mut path = HashSet::new();
        let mut oriented_path = HashSet::new();

        loop {
            if self.map.len() < 50 {
                self.screen(&path, &gard);
                println!();
                println!();
                sleep(Duration::from_millis(25));
            }

            path.insert((gard.x, gard.y));
            oriented_path.insert(gard.clone());
            let mut next = gard.clone();
            match gard.direction {
                Direction::North => {
                    if next.x == 0 {
                        return Some(path);
                    }
                    next.x -= 1;
                }
                Direction::Est => next.y += 1,
                Direction::South => next.x += 1,
                Direction::West => {
                    if next.y == 0 {
                        return Some(path);
                    }
                    next.y -= 1;
                }
            }

            if self.map.get(next.x).is_none() || self.map[next.x].get(next.y).is_none() {
                break;
            }

            if self.map[next.x][next.y] == Tile::Obstacle {
                gard.direction = gard.direction.next()
            } else {
                gard.x = next.x;
                gard.y = next.y;
            }

            if oriented_path.contains(&gard) {
                return None;
            }
        }
        Some(path)
    }
}
