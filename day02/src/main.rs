use helper::{InputReader, Solver};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: 2.to_string(),
        result1: Some("572".to_string()),
        example2: Some("4".to_string()),
        result2: Some("612".to_string()),
        kind: Default::default(),
    };

    solver.solve("day02");
}

#[derive(Debug)]
struct Row {
    list: Vec<u32>,
    safe: bool,
}

#[derive(Debug, Default)]
struct Container {
    data: Vec<Row>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        let list: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

        if list.is_empty() {
            return;
        }

        self.data.push(Row {
            safe: is_safe(list.clone()),
            list,
        })
    }

    fn star1(&self) -> String {
        self.data.iter().filter(|a| a.safe).count().to_string()
    }

    fn star2(&self) -> String {
        let mut nb = self.data.iter().filter(|a| a.safe).count();

        let to_check: Vec<&Row> = self.data.iter().filter(|a| !a.safe).collect();

        for row in to_check {
            for n in 0..row.list.len() {
                let mut list = row.list.clone();

                list.remove(n);

                if is_safe(list) {
                    nb += 1;
                    break;
                }
            }
        }

        nb.to_string()
    }
}

fn is_safe(list: Vec<u32>) -> bool {
    let mut reverse = list.clone();
    reverse.reverse();
    if !list.is_sorted() && !reverse.is_sorted() {
        return false;
    }

    for w in list.windows(2) {
        if w[0] == w[1] {
            return false;
        }
        if w[0].abs_diff(w[1]) > 3 {
            return false;
        }
    }

    true
}
