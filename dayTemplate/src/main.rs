use helper::{aoc1, aoc2, InputReader};
use regex::Regex;

fn main() {
    let example1 = 0;
    let result1 = 0;
    let example2 = 0;

    aoc1!(Container, "dayTemplate", example1);
    aoc2!(Container, "dayTemplate", example1, result1, example2);
}

#[derive(Debug)]
struct Container {
    #[allow(dead_code)]
    regex: Regex,
}
impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^(.*)$").unwrap(),
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, _line: &str) {
        todo!("add_line")
    }

    fn star1(self) -> String {
        todo!("star1")
    }

    fn star2(self) -> String {
        todo!("star2")
    }
}
