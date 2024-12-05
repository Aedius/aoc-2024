use helper::{InputReader, Solver};
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "143".to_string(),
        result1: Some("4790".to_string()),
        example2: Some("123".to_string()),
        result2: Some("6319".to_string()),
        kind: Default::default(),
    };

    solver.solve("day05");
}

#[derive(Debug, Default)]
struct Container {
    rules: HashMap<u32, Vec<u32>>,
    prints: Vec<Vec<u32>>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if line.contains(',') {
            self.prints
                .push(line.split(',').map(|v| v.parse().unwrap()).collect())
        } else if line.contains('|') {
            let d: Vec<u32> = line.split('|').map(|v| v.parse().unwrap()).collect();

            let r = self.rules.entry(d[0]).or_default();
            r.push(d[1])
        }
    }

    fn star1(&self) -> String {
        let mut res = 0;

        for print in &self.prints {
            if self.is_print_ordered(print) {
                res += print[(print.len() - 1) / 2];
            }
        }

        res.to_string()
    }

    fn star2(&self) -> String {
        let mut res = 0;

        for print in &self.prints {
            if !self.is_print_ordered(print) {
                let sorted = self.sort(print.clone());

                res += sorted[(sorted.len() - 1) / 2];
            }
        }
        res.to_string()
    }
}

impl Container {
    fn is_print_ordered(&self, print: &[u32]) -> bool {
        for i in 1..print.len() {
            if let Some(checks) = self.rules.get(&print[i]) {
                for check in checks {
                    if print[0..i].contains(check) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn sort(&self, mut print: Vec<u32>) -> Vec<u32> {
        print.sort_by(|a, b| {
            if let Some(list) = self.rules.get(a) {
                if list.contains(b) {
                    return Ordering::Less;
                }
            }
            if let Some(list) = self.rules.get(b) {
                if list.contains(a) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });

        print
    }
}
