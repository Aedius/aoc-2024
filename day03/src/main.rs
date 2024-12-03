use helper::{InputReader, Solver};
use regex::Regex;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "161".to_string(),
        result1: Some("184576302".to_string()),
        example2: Some("48".to_string()),
        result2: Some("118173507".to_string()),
        kind: Default::default(),
    };

    solver.solve("day03");
}

#[derive(Debug)]
struct Container {
    #[allow(dead_code)]
    regex: Regex,
    mul: Vec<(usize, usize, bool)>,
    enable: bool,
}
impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"(do(\()(\))|don't(\()(\))|mul\((\d+),(\d+)\))").unwrap(),
            mul: vec![],
            enable: true,
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        for (_, [inst, a, b]) in self.regex.captures_iter(line).map(|c| c.extract()) {
            if inst == "do()" {
                self.enable = true
            } else if inst == "don't()" {
                self.enable = false
            } else {
                self.mul
                    .push((a.parse().unwrap(), b.parse().unwrap(), self.enable))
            }
        }
    }

    fn star1(&self) -> String {
        let mut res = 0;

        for couple in self.mul.clone() {
            res += couple.0 * couple.1
        }

        res.to_string()
    }

    fn star2(&self) -> String {
        let mut res = 0;

        for couple in self.mul.clone() {
            if couple.2 {
                res += couple.0 * couple.1
            }
        }

        res.to_string()
    }
}
