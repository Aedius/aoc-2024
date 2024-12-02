use helper::{InputReader, Solver};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: 11.to_string(),
        result1: Some("2264607".to_string()),
        example2: Some("31".to_string()),
        result2: Some("19457120".to_string()),
        kind: Default::default(),
    };

    solver.solve("day01");
}

#[derive(Debug)]
struct Container {
    regex: Regex,
    list1: Vec<u32>,
    list2: Vec<u32>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^([\d]+) +([\d]+)$").unwrap(),
            list1: vec![],
            list2: vec![],
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if let Some(cap) = self.regex.captures(line) {
            self.list1.push(cap[1].parse::<u32>().unwrap());
            self.list2.push(cap[2].parse::<u32>().unwrap());
        }
    }

    fn star1(&self) -> String {
        let mut list1 = self.list1.clone();
        list1.sort();
        let mut list1 = list1.iter();

        let mut list2 = self.list2.clone();
        list2.sort();
        let mut list2 = list2.iter();

        let mut result = 0;

        while let (Some(left), Some(right)) = (list1.next(), list2.next()) {
            result += left.abs_diff(*right);
        }

        result.to_string()
    }

    fn star2(&self) -> String {
        let hash1 = Self::get_hash_map(self.list1.clone());
        let hash2 = Self::get_hash_map(self.list2.clone());

        let mut result = 0;

        for (k, v) in hash1.iter() {
            result += k * v * hash2.get(k).unwrap_or(&0);
        }

        result.to_string()
    }
}

impl Container {
    fn get_hash_map(list: Vec<u32>) -> HashMap<u32, u32> {
        let mut hash = HashMap::new();
        for n in list.iter() {
            let entry = hash.entry(*n).or_insert(0);
            *entry += 1;
        }
        hash
    }
}
