use helper::{InputReader, Solver};
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "55312".to_string(),
        result1: Some("197157".to_string()),
        example2: Some("65601038650482".to_string()),
        result2: Some("234430066982597".to_string()),
        kind: Default::default(),
    };

    solver.solve("day11");
}

#[derive(Debug, Default)]
struct Container {
    list: Vec<usize>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.list = line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
    }

    fn star1(&self) -> String {
        let mut res = self.list.clone();
        for _ in 0..25 {
            res = blink(res);
        }
        res.len().to_string()
    }

    fn star2(&self) -> String {
        let mut res = vec_to_hash(self.list.clone());

        for _ in 0..75 {
            res = blink_fast(res);
        }

        let mut count = 0;
        for (_, nb) in res {
            count += nb;
        }

        count.to_string()
    }
}

fn vec_to_hash(origin: Vec<usize>) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for v in origin {
        let a = res.entry(v).or_default();
        *a += 1;
    }
    res
}

fn blink(origin: Vec<usize>) -> Vec<usize> {
    let mut destination = Vec::with_capacity(origin.len() * 2);

    for v in origin {
        match v.checked_ilog10() {
            None => destination.push(1),
            Some(n) => {
                if n % 2 == 1 {
                    let power = n / 2 + 1;
                    let tens: usize = (10_u32.pow(power)).try_into().unwrap();
                    destination.push(v / tens);
                    destination.push(v % tens)
                } else {
                    destination.push(v * 2024)
                }
            }
        }
    }

    destination
}

fn blink_fast(origin: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut destination = HashMap::with_capacity(origin.len() * 2);

    for (v, nb) in origin {
        match v.checked_ilog10() {
            None => {
                let a = destination.entry(1).or_default();
                *a += nb;
            }
            Some(n) => {
                if n % 2 == 1 {
                    let power = n / 2 + 1;
                    let tens: usize = (10_u32.pow(power)).try_into().unwrap();

                    let a = destination.entry(v / tens).or_default();
                    *a += nb;
                    let a = destination.entry(v % tens).or_default();
                    *a += nb;
                } else {
                    let a = destination.entry(v * 2024).or_default();
                    *a += nb;
                }
            }
        }
    }

    destination
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut val1 = vec![125, 17];
        let mut val2 = vec_to_hash(val1.clone());
        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(val1, vec![253000, 1, 7]);
        assert_eq!(val2, vec_to_hash(val1.clone()));

        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(val1, vec![253, 0, 2024, 14168]);
        assert_eq!(val2, vec_to_hash(val1.clone()));

        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(val1, vec![512072, 1, 20, 24, 28676032]);
        assert_eq!(val2, vec_to_hash(val1.clone()));

        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(val1, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        assert_eq!(val2, vec_to_hash(val1.clone()));

        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(
            val1,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(val2, vec_to_hash(val1.clone()));

        val1 = blink(val1);
        val2 = blink_fast(val2);
        assert_eq!(
            val1,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
        assert_eq!(val2, vec_to_hash(val1.clone()));
    }
}
