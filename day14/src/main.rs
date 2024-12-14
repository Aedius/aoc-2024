use helper::{InputReader, Solver};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "12".to_string(),
        result1: Some("229069152".to_string()),
        example2: Some("no easter eggs".to_string()),
        result2: Some("7383".to_string()),
        kind: Default::default(),
    };

    solver.solve("day14");
}

#[derive(Debug, Clone)]
struct Container {
    #[allow(dead_code)]
    regex: Regex,
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}
impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^p=(\d+),(\d+) v=([-\d]+),([-\d]+)$").unwrap(),
            robots: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone)]
struct Velocity {
    x: isize,
    y: isize,
}
#[derive(Debug, Clone)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        if self.robots.len() < 20 {
            self.width = 11;
            self.height = 7;
        } else {
            self.width = 101;
            self.height = 103;
        }
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(cap) = self.regex.captures(line) {
            self.robots.push(Robot {
                pos: Position {
                    x: cap[1].parse::<usize>().unwrap(),
                    y: cap[2].parse::<usize>().unwrap(),
                },
                vel: Velocity {
                    x: cap[3].parse::<isize>().unwrap(),
                    y: cap[4].parse::<isize>().unwrap(),
                },
            })
        } else {
            panic!("cannot load row")
        }
    }

    fn star1(&self) -> String {
        let mut container = self.clone();
        container.tic(100);

        let (a, b, c, d) = container.security();
        (a * b * c * d).to_string()
    }

    fn star2(&self) -> String {
        if self.width < 20 {
            return "no easter eggs".to_string();
        }
        let mut container = self.clone();
        let mut res = 0;
        loop {
            container.tic(1);
            res += 1;
            let (a, b, c, d) = container.security();

            let list = [a, b, c, d];
            let max = list.iter().max().unwrap();
            let sum = list.iter().sum::<usize>() * 2 / 3;
            if *max > sum {
                container.display();
                break;
            }
        }
        res.to_string()
    }
}

impl Container {
    fn tic(&mut self, nb: usize) {
        for robot in self.robots.iter_mut() {
            let vel_x: usize = (robot.vel.x + self.width as isize) as usize % self.width;
            robot.pos.x = (robot.pos.x + vel_x * nb) % self.width;

            let vel_y: usize = (robot.vel.y + self.height as isize) as usize % self.height;
            robot.pos.y = (robot.pos.y + vel_y * nb) % self.height;
        }
    }

    fn security(&self) -> (usize, usize, usize, usize) {
        let w = self.width / 2;
        let h = self.height / 2;

        let mut quarter_top_left = 0;
        let mut quarter_top_right = 0;
        let mut quarter_bottom_left = 0;
        let mut quarter_bottom_right = 0;

        for robot in &self.robots {
            match robot.pos.x.cmp(&w) {
                Ordering::Less => match robot.pos.y.cmp(&h) {
                    Ordering::Less => {
                        quarter_top_left += 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        quarter_bottom_left += 1;
                    }
                },
                Ordering::Equal => {}
                Ordering::Greater => match robot.pos.y.cmp(&h) {
                    Ordering::Less => {
                        quarter_top_right += 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        quarter_bottom_right += 1;
                    }
                },
            }
        }
        (
            quarter_top_left,
            quarter_top_right,
            quarter_bottom_left,
            quarter_bottom_right,
        )
    }

    fn display(&self) {
        let mut img = self.get_img();
        for y in 0..self.height {
            for x in 0..self.width {
                match img.entry((x, y)) {
                    Entry::Occupied(val) => {
                        print!("{}", val.get())
                    }
                    Entry::Vacant(_) => {
                        print!(".")
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn get_img(&self) -> HashMap<(usize, usize), usize> {
        let mut img: HashMap<(usize, usize), usize> = HashMap::new();
        for robot in &self.robots {
            let pixel = img.entry((robot.pos.x, robot.pos.y)).or_default();
            *pixel += 1;
        }
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_robot() {
        let mut container = Container {
            regex: Regex::new(r"^.*$").unwrap(),
            robots: vec![Robot {
                pos: Position { x: 2, y: 4 },
                vel: Velocity { x: 2, y: -3 },
            }],
            width: 11,
            height: 7,
        };
        container.display();

        container.tic(1);
        assert_eq!(container.robots[0].pos, Position { x: 4, y: 1 });
        container.display();

        container.tic(1);
        assert_eq!(container.robots[0].pos, Position { x: 6, y: 5 });
        container.display();

        container.tic(1);
        assert_eq!(container.robots[0].pos, Position { x: 8, y: 2 });
        container.display();

        container.tic(1);
        assert_eq!(container.robots[0].pos, Position { x: 10, y: 6 });
        container.display();

        container.tic(1);
        assert_eq!(container.robots[0].pos, Position { x: 1, y: 3 });
        container.display();
    }
}
