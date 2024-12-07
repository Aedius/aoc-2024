use helper::{InputReader, Solver};
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "3749".to_string(),
        result1: Some("882304362421".to_string()),
        example2: Some("11387".to_string()),
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day07");
}

#[derive(Debug)]
struct Container {
    #[allow(dead_code)]
    regex: Regex,
    equations: Vec<Equation>,
}

#[derive(Debug)]
struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^(\d+): ((\d| )+)$").unwrap(),
            equations: Vec::new(),
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if let Some(cap) = self.regex.captures(line) {
            let result = &cap[1].parse::<usize>().unwrap();

            let values: &Vec<usize> = &cap[2]
                .split(' ')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            self.equations.push(Equation {
                result: *result,
                values: values.clone(),
            })
        }
    }

    fn star1(&self) -> String {
        let mut result = 0;

        for equation in &self.equations {
            let mut list: VecDeque<usize> = equation.values.clone().into();

            let possible = compute_possible_star1(list.pop_front().unwrap(), list);

            if possible.contains(&equation.result) {
                result += equation.result;
            }
        }

        result.to_string()
    }

    fn star2(&self) -> String {
        let mut result = 0;

        for equation in &self.equations {
            let mut list: VecDeque<usize> = equation.values.clone().into();

            let possible = compute_possible_star2(list.pop_front().unwrap(), list);

            if possible.contains(&equation.result) {
                result += equation.result;
            }
        }

        result.to_string()
    }
}

fn compute_possible_star1(current: usize, mut other: VecDeque<usize>) -> Vec<usize> {
    if other.is_empty() {
        return vec![current];
    }

    let next = other.pop_front().unwrap();

    let mut plus = compute_possible_star1(current + next, other.clone());
    let mut mult = compute_possible_star1(current * next, other);

    plus.append(&mut mult);

    plus
}
fn compute_possible_star2(current: usize, mut other: VecDeque<usize>) -> Vec<usize> {
    if other.is_empty() {
        return vec![current];
    }

    let next = other.pop_front().unwrap();

    let mut plus = compute_possible_star2(current + next, other.clone());
    let mut mult = compute_possible_star2(current * next, other.clone());
    let mut conc =
        compute_possible_star2(format!("{current}{next}").parse::<usize>().unwrap(), other);

    plus.append(&mut mult);
    plus.append(&mut conc);

    plus
}
