use helper::{InputReader, Solver};
use regex::Regex;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "".to_string(),
        result1: None,
        example2: None,
        result2: None,
        kind: Default::default(),
    };

    solver.solve("dayTemplate");
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
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        todo!("add_line")
    }

    fn star1(&self) -> String {
        dbg!(self);

        todo!("star1")
    }

    fn star2(&self) -> String {
        todo!("star2")
    }
}
