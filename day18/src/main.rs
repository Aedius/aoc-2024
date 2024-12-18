use helper::{InputReader, Solver};
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "22".to_string(),
        result1: Some(438.to_string()),
        example2: Some("6,1".to_string()),
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day18");
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Default)]
enum Kind {
    #[default]
    Safe,
    Corrupted(usize),
    Distance(usize),
}

#[derive(Default, Debug, Clone)]
struct Container {
    map: HashMap<Pos, Kind>,
    size: usize,
    time: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.time += 1;
        let item: Vec<usize> = line
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        self.map.insert(
            Pos {
                x: item[0],
                y: item[1],
            },
            Kind::Corrupted(self.time),
        );
        self.size = self.size.max(item[0]);
    }

    fn star1(&self) -> String {
        let mut container = self.clone();
        let end = Pos {
            x: container.size,
            y: container.size,
        };
        let mut heads = vec![Pos { x: 0, y: 0 }];
        let mut step = 0;

        let k = if self.size == 6 { 12 } else { 1024 };

        while container.map.get(&end).is_none() {
            container.display_debug();
            heads = container.next(heads.clone(), &mut step, k);
        }

        container.display_debug();

        let val = container.map.get(&end).unwrap();
        match val {
            Kind::Safe => {
                todo!("safe ?")
            }
            Kind::Corrupted(_) => {
                todo!("corrupted ?")
            }
            Kind::Distance(n) => n.to_string(),
        }
    }

    fn star2(&self) -> String {
        let mut k = if self.size == 6 { 12 } else { 1024 };

        'k: loop {
            let mut container = self.clone();
            let end = Pos {
                x: container.size,
                y: container.size,
            };

            let mut heads = vec![Pos { x: 0, y: 0 }];
            let mut step = 0;

            while container.map.get(&end).is_none() {
                container.display_debug();
                heads = container.next(heads.clone(), &mut step, k);
                if heads.is_empty() {
                    break 'k;
                }
            }

            k += 1;
        }

        for (pos, tile) in self.map.clone() {
            match tile {
                Kind::Safe => {}
                Kind::Corrupted(n) => {
                    if n == k {
                        return format!("{},{}", pos.x, pos.y);
                    }
                }
                Kind::Distance(_) => {}
            }
        }

        panic!("tile not found")
    }
}

impl Container {
    fn next(&mut self, heads: Vec<Pos>, step: &mut usize, k: usize) -> Vec<Pos> {
        let mut next = Vec::new();
        for head in heads {
            let entry = self.map.entry(head).or_default();
            {
                match entry {
                    Kind::Safe => {
                        *entry = Kind::Distance(*step);
                        self.get_nexts(&mut next, head);
                    }
                    Kind::Corrupted(n) => {
                        if *n > k {
                            *entry = Kind::Distance(*step);
                            self.get_nexts(&mut next, head);
                        }
                    }
                    Kind::Distance(_) => {
                        // already used
                    }
                }
            }
        }
        *step += 1;

        next
    }

    fn get_nexts(&mut self, next: &mut Vec<Pos>, head: Pos) {
        if head.x < self.size {
            next.push(Pos {
                x: head.x + 1,
                y: head.y,
            });
        }
        if head.x > 0 {
            next.push(Pos {
                x: head.x - 1,
                y: head.y,
            });
        }
        if head.y < self.size {
            next.push(Pos {
                x: head.x,
                y: head.y + 1,
            });
        }
        if head.y > 0 {
            next.push(Pos {
                x: head.x,
                y: head.y - 1,
            });
        }
    }

    fn display_debug(&self) {
        if self.size != 6 {
            return;
        }

        for y in 0..self.size + 1 {
            for x in 0..self.size + 1 {
                match self.map.get(&Pos { x, y }) {
                    None => {
                        print!("----")
                    }
                    Some(Kind::Corrupted(_)) => {
                        print!(" ## ")
                    }
                    Some(Kind::Distance(n)) => {
                        print!(" {n:02} ")
                    }
                    Some(Kind::Safe) => {
                        print!(" .. ")
                    }
                }
            }
            println!()
        }
        println!();
    }
}
