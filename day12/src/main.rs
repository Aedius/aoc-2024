use helper::{InputReader, Solver};
use std::cmp::PartialEq;
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "1930".to_string(),
        result1: None,
        example2: None,
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day12");
}

#[derive(Debug, Default, Clone, PartialEq, Ord, PartialOrd, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct Container {
    field: Vec<Vec<(Point, String)>>,
    zone: HashMap<String, Vec<Point>>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let val = c.to_string();
            let point = Point {
                x: x.try_into().unwrap(),
                y: self.field.len().try_into().unwrap(),
            };
            row.push((point.clone(), val.clone()));

            let zone = self.zone.entry(val).or_default();
            zone.push(point);
        }
        self.field.push(row)
    }

    fn star1(&self) -> String {
        let mut res = 0;

        for (key, val) in self.zone.iter() {
            let list = contiguous(val.clone());
            for zone in list {
                let area = zone.len();
                let perimeter = perimeter(&zone);
                println!("{key} : area {area}, perimeter {perimeter}");
                res += area * perimeter;
            }
        }
        res.to_string()
    }

    fn star2(&self) -> String {
        todo!("star2")
    }
}

impl Container {}

fn contiguous(mut input: Vec<Point>) -> Vec<Vec<Point>> {
    let mut result = vec![];
    while let Some(check) = input.pop() {
        let list = all_neighbor(input.clone(), &check);
        for l in &list {
            input.retain(|p| p != l)
        }
        result.push(list);
    }
    result
}

fn all_neighbor(input: Vec<Point>, p: &Point) -> Vec<Point> {
    if input.is_empty() {
        return vec![p.clone()];
    }
    let mut res = vec![p.clone()];
    let neighbour = get_neighbor(p, &input);

    let mut no_neighbour = input.clone();

    for n in &neighbour {
        no_neighbour.retain(|p| p != n);
        res.push(n.clone());
    }

    for n in &neighbour {
        let mut an = all_neighbor(no_neighbour.clone(), n);
        res.append(&mut an);
    }

    res.sort();
    res.dedup();

    res
}

fn perimeter(input: &Vec<Point>) -> usize {
    let mut res = 0;
    for p in input {
        res += nb_perimeter(p, input);
    }

    res
}
fn nb_perimeter(p: &Point, input: &Vec<Point>) -> usize {
    4 - get_neighbor(p, input).len()
}

fn get_neighbor(p: &Point, input: &Vec<Point>) -> Vec<Point> {
    let mut res = Vec::with_capacity(4);

    let check = vec![
        Point { x: p.x + 1, y: p.y },
        Point { x: p.x - 1, y: p.y },
        Point { x: p.x, y: p.y + 1 },
        Point { x: p.x, y: p.y - 1 },
    ];
    for c in check {
        if input.contains(&c) {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighour() {
        assert_eq!(all_neighbor(vec![], &Point { x: 0, y: 0 }), vec![Point { x: 0, y: 0 }]);
        assert_eq!(
            all_neighbor(
                vec![Point { x: 1, y: 0 }, Point { x: 2, y: 0 }],
                &Point { x: 0, y: 0 }
            ),
            vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }]
        );
    }

    #[test]
    fn smaller_example() {
        let mut container = Container::default();
        container.add_line("AAAA");
        container.add_line("BBCD");
        container.add_line("BBCC");
        container.add_line("EEEC");

        assert_eq!(container.star1(), "140".to_string());
    }

    #[test]
    fn not_contiguous_example() {
        let mut container = Container::default();
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");

        assert_eq!(container.star1(), "772".to_string());
    }
}
