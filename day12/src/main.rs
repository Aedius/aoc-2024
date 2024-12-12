use helper::{InputReader, Solver};
use std::cmp::PartialEq;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::Sum;

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
    kind: String,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl<'s> Sum<&'s Point> for usize {
    fn sum<I: Iterator<Item = &'s Point>>(iter: I) -> Self {
        let mut res = 0;
        for p in iter {
            if p.north {
                res += 1;
            }
            if p.east {
                res += 1;
            }
            if p.south {
                res += 1;
            }
            if p.west {
                res += 1;
            }
        }
        res
    }
}

#[derive(Debug, Default, Clone)]
struct Container {
    data: HashMap<(isize, isize), Point>,
    width: usize,
    height: usize,
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        for x in 0..self.width {
            match self
                .data
                .entry((x.try_into().unwrap(), (self.height - 1).try_into().unwrap()))
            {
                Entry::Occupied(mut p) => {
                    let n = p.get_mut();
                    n.south = true;
                }
                Entry::Vacant(_) => {
                    todo!("no vacant allowed");
                }
            };
        }
        for y in 0..self.width {
            match self
                .data
                .entry(((self.width - 1).try_into().unwrap(), y.try_into().unwrap()))
            {
                Entry::Occupied(mut p) => {
                    let n = p.get_mut();
                    n.east = true;
                }
                Entry::Vacant(_) => {
                    todo!("no vacant allowed");
                }
            };
        }
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.width = line.len();
        for (x, c) in line.chars().enumerate() {
            let mut np = Point {
                x: x.try_into().unwrap(),
                y: self.height.try_into().unwrap(),
                kind: c.to_string(),
                north: false,
                east: false,
                south: false,
                west: false,
            };
            match self.data.entry((np.x - 1, np.y)) {
                Entry::Occupied(mut occupied) => {
                    let neighbor = occupied.get_mut();
                    if neighbor.kind != np.kind {
                        neighbor.east = true;
                        np.west = true;
                    }
                }
                Entry::Vacant(_) => {
                    np.west = true;
                }
            }
            match self.data.entry((np.x, np.y - 1)) {
                Entry::Occupied(mut occupied) => {
                    let neighbor = occupied.get_mut();
                    if neighbor.kind != np.kind {
                        neighbor.south = true;
                        np.north = true;
                    }
                }
                Entry::Vacant(_) => {
                    np.north = true;
                }
            }

            self.data.insert((np.x, np.y), np);
        }
        self.height += 1;
    }

    fn star1(&self) -> String {
        let mut res: usize = 0;

        let mut container = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                match container
                    .data
                    .entry((x.try_into().unwrap(), y.try_into().unwrap()))
                {
                    Entry::Occupied(occupied) => {
                        let point = occupied.remove();

                        let zone = container.get_zone(point.clone());
                        let perimeter: usize = zone.iter().sum();
                        let area: usize = zone.len();
                        res += area * perimeter;
                        println!(
                            "A region of {} : plants with price {area} * {perimeter} = {}",
                            point.kind,
                            area * perimeter
                        );
                    }
                    Entry::Vacant(_) => {
                        // already taken
                    }
                }
            }
        }

        res.to_string()
    }

    fn star2(&self) -> String {
        todo!("star2")
    }
}

impl Container {
    fn get_zone(&mut self, z: Point) -> Vec<Point> {
        let mut to_check = vec![z];

        let mut res = vec![];

        while !to_check.is_empty() {
            let mut next = vec![];

            for check in to_check {
                if !check.south {
                    match self.data.entry((check.x, check.y + 1)) {
                        Entry::Occupied(occ) => {
                            let point = occ.remove();
                            next.push(point);
                        }
                        Entry::Vacant(_) => {}
                    }
                }
                if !check.north {
                    match self.data.entry((check.x, check.y - 1)) {
                        Entry::Occupied(occ) => {
                            let point = occ.remove();
                            next.push(point);
                        }
                        Entry::Vacant(_) => {}
                    }
                }
                if !check.east {
                    match self.data.entry((check.x + 1, check.y)) {
                        Entry::Occupied(occ) => {
                            let point = occ.remove();
                            next.push(point);
                        }
                        Entry::Vacant(_) => {}
                    }
                }
                if !check.west {
                    match self.data.entry((check.x - 1, check.y)) {
                        Entry::Occupied(occ) => {
                            let point = occ.remove();
                            next.push(point);
                        }
                        Entry::Vacant(_) => {}
                    }
                }
                res.push(check);
            }

            to_check = next;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_example() {
        let mut container = Container::default();
        container.add_line("AAAA");
        container.add_line("BBCD");
        container.add_line("BBCC");
        container.add_line("EEEC");
        container.after_all_line();

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
        container.after_all_line();

        assert_eq!(container.star1(), "772".to_string());
    }
}
