use helper::{InputReader, Solver};
use num::integer::lcm;
use regex::Regex;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "480".to_string(),
        result1: Some("33481".to_string()),
        example2: Some("875318608908".to_string()),
        result2: Some("92572057880885".to_string()),
        kind: Default::default(),
    };

    solver.solve("day13");
}

#[derive(Debug)]
struct Container {
    regex_prize: Regex,
    regex_button: Regex,
    previous_lines: Vec<String>,
    systems: Vec<System>,
}

#[derive(Debug, Clone)]
struct Equation {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct System {
    first: Equation,
    second: Equation,
}

#[derive(Debug, Eq, PartialEq)]
struct Solution {
    x: usize,
    y: usize,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex_prize: Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap(),
            regex_button: Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap(),
            previous_lines: Vec::with_capacity(2),
            systems: Vec::new(),
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.previous_lines.len() < 2 {
            self.previous_lines.push(line.to_string());
            return;
        }
        let prize1: usize;
        let prize2: usize;
        let button_a1: usize;
        let button_a2: usize;
        let button_b1: usize;
        let button_b2: usize;

        if let Some(cap) = self.regex_prize.captures(line) {
            prize1 = cap[1].parse::<usize>().unwrap();
            prize2 = cap[2].parse::<usize>().unwrap();
        } else {
            panic!("cannot load prizes")
        }
        if let Some(cap) = self.regex_button.captures(&self.previous_lines[0]) {
            button_a1 = cap[1].parse::<usize>().unwrap();
            button_a2 = cap[2].parse::<usize>().unwrap();
        } else {
            panic!("cannot data A")
        }
        if let Some(cap) = self.regex_button.captures(&self.previous_lines[1]) {
            button_b1 = cap[1].parse::<usize>().unwrap();
            button_b2 = cap[2].parse::<usize>().unwrap();
        } else {
            panic!("cannot data A")
        }
        self.systems.push(System {
            first: Equation {
                a: button_a1,
                b: button_b1,
                c: prize1,
            },
            second: Equation {
                a: button_a2,
                b: button_b2,
                c: prize2,
            },
        });
        self.previous_lines = Vec::with_capacity(2);
    }

    fn star1(&self) -> String {
        let mut res = 0;

        for sys in &self.systems {
            if let Some(sol) = sys.compute_solution() {
                res += 3 * sol.x + sol.y;
            }
        }

        res.to_string()
    }

    fn star2(&self) -> String {
        let mut res = 0;

        for sml in &self.systems {
            if let Some(sol) = sml.to_big().compute_solution() {
                res += 3 * sol.x + sol.y;
            }
        }

        res.to_string()
    }
}

impl System {
    fn compute_solution(&self) -> Option<Solution> {
        if self.first.a == self.second.a {
            let sys = System {
                first: Equation {
                    a: self.first.b,
                    b: self.first.a,
                    c: self.first.c,
                },
                second: Equation {
                    a: self.second.b,
                    b: self.second.a,
                    c: self.second.c,
                },
            };
            return sys
                .compute_solution()
                .map(|sol| Solution { x: sol.y, y: sol.x });
        }

        let ppcm_x = lcm(self.first.a, self.second.a);

        let normalized_sys = System {
            first: Equation {
                a: ppcm_x,
                b: self.first.b * (ppcm_x / self.first.a),
                c: self.first.c * (ppcm_x / self.first.a),
            },
            second: Equation {
                a: ppcm_x,
                b: self.second.b * (ppcm_x / self.second.a),
                c: self.second.c * (ppcm_x / self.second.a),
            },
        };

        let b: isize = normalized_sys.first.b as isize - normalized_sys.second.b as isize;
        let c: isize = normalized_sys.first.c as isize - normalized_sys.second.c as isize;

        if c % b == 0 {
            let y: usize = (c / b).try_into().unwrap();
            let x = (normalized_sys.first.c - normalized_sys.first.b * y) / ppcm_x;

            let solution = Solution { x, y };

            assert_eq!(
                self.first.a * solution.x + self.first.b * solution.y,
                self.first.c
            );
            assert_eq!(
                self.second.a * solution.x + self.second.b * solution.y,
                self.second.c
            );

            return Some(solution);
        }

        None
    }

    fn to_big(&self) -> Self {
        let mut big = self.clone();
        big.first.c += 10000000000000;
        big.second.c += 10000000000000;
        big
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_1() {
        let system = System {
            first: Equation {
                a: 94,
                b: 22,
                c: 8400,
            },
            second: Equation {
                a: 34,
                b: 67,
                c: 5400,
            },
        };

        assert_eq!(system.compute_solution(), Some(Solution { x: 80, y: 40 }));

        let system = system.to_big();

        assert!(system.compute_solution().is_none())
    }
    #[test]
    fn system_2() {
        let system = System {
            first: Equation {
                a: 26,
                b: 67,
                c: 12748,
            },
            second: Equation {
                a: 66,
                b: 21,
                c: 12176,
            },
        };

        assert_eq!(system.compute_solution(), None);
        let system = system.to_big();
        assert!(system.compute_solution().is_some())
    }
    #[test]
    fn system_3() {
        let system = System {
            first: Equation {
                a: 17,
                b: 84,
                c: 7870,
            },
            second: Equation {
                a: 86,
                b: 37,
                c: 6450,
            },
        };

        assert_eq!(system.compute_solution(), Some(Solution { x: 38, y: 86 }));
        let system = system.to_big();

        assert!(system.compute_solution().is_none())
    }
    #[test]
    fn system_4() {
        let system = System {
            first: Equation {
                a: 69,
                b: 27,
                c: 18641,
            },
            second: Equation {
                a: 23,
                b: 71,
                c: 10279,
            },
        };

        assert_eq!(system.compute_solution(), None);
        let system = system.to_big();

        assert!(system.compute_solution().is_some())
    }
}
