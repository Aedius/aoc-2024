use helper::{InputReader, Solver};
use std::cmp::PartialEq;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::Sum;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "1930".to_string(),
        result1: Some("1477762".to_string()),
        example2: Some("1206".to_string()),
        result2: Some("923480".to_string()),
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
        self.star(|zone| zone.iter().sum())
    }

    fn star2(&self) -> String {
        self.star(|zone| {
            let mut zone = zone.clone();
            zone.retain(|p| p.west || p.east || p.north || p.south);

            let mut north: HashMap<isize, Vec<Point>> = HashMap::new();
            let mut south: HashMap<isize, Vec<Point>> = HashMap::new();
            let mut east: HashMap<isize, Vec<Point>> = HashMap::new();
            let mut west: HashMap<isize, Vec<Point>> = HashMap::new();

            for p in &zone {
                if p.north {
                    let l = north.entry(p.y).or_default();
                    l.push(p.clone())
                }
                if p.south {
                    let l = south.entry(p.y).or_default();
                    l.push(p.clone())
                }
                if p.east {
                    let l = east.entry(p.x).or_default();
                    l.push(p.clone())
                }
                if p.west {
                    let l = west.entry(p.x).or_default();
                    l.push(p.clone())
                }
            }

            let mut wall_north = 0;

            for (_, mut list) in north {
                list.sort_by(|a, b| a.x.cmp(&b.x));
                let mut wall = 1;
                for w in list.windows(2) {
                    if w[0].x + 1 != w[1].x {
                        wall += 1;
                    }
                }
                wall_north += wall;
            }

            let mut wall_south = 0;

            for (_, mut list) in south {
                list.sort_by(|a, b| a.x.cmp(&b.x));
                let mut wall = 1;
                for w in list.windows(2) {
                    if w[0].x + 1 != w[1].x {
                        wall += 1;
                    }
                }
                wall_south += wall;
            }

            let mut wall_east = 0;

            for (_, mut list) in east {
                list.sort_by(|a, b| a.y.cmp(&b.y));
                let mut wall = 1;
                for w in list.windows(2) {
                    if w[0].y + 1 != w[1].y {
                        wall += 1;
                    }
                }
                wall_east += wall;
            }

            let mut wall_west = 0;

            for (_, mut list) in west {
                list.sort_by(|a, b| a.y.cmp(&b.y));
                let mut wall = 1;
                for w in list.windows(2) {
                    if w[0].y + 1 != w[1].y {
                        wall += 1;
                    }
                }
                wall_west += wall;
            }

            wall_north + wall_south + wall_east + wall_west
        })
    }
}

impl Container {
    fn star(&self, perimeter: fn(&Vec<Point>) -> usize) -> String {
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
                        let perimeter: usize = perimeter(&zone);
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
    fn smaller_example_1() {
        let mut container = Container::default();
        container.add_line("AAAA");
        container.add_line("BBCD");
        container.add_line("BBCC");
        container.add_line("EEEC");
        container.after_all_line();

        assert_eq!(container.star1(), "140".to_string());
    }
    #[test]
    fn smaller_example_2() {
        let mut container = Container::default();
        container.add_line("AAAA");
        container.add_line("BBCD");
        container.add_line("BBCC");
        container.add_line("EEEC");
        container.after_all_line();

        assert_eq!(container.star2(), "80".to_string());
    }

    #[test]
    fn not_contiguous_example_1() {
        let mut container = Container::default();
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");
        container.after_all_line();

        assert_eq!(container.star1(), "772".to_string());
    }
    #[test]
    fn not_contiguous_example_2() {
        let mut container = Container::default();
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");
        container.add_line("OXOXO");
        container.add_line("OOOOO");
        container.after_all_line();

        assert_eq!(container.star2(), "436".to_string());
    }

    #[test]
    fn e_shape_example() {
        let mut container = Container::default();
        container.add_line("EEEEE");
        container.add_line("EXXXX");
        container.add_line("EEEEE");
        container.add_line("EXXXX");
        container.add_line("EEEEE");
        container.after_all_line();

        assert_eq!(container.star2(), "236".to_string());
    }

    #[test]
    fn abba_example() {
        let mut container = Container::default();
        container.add_line("AAAAAA");
        container.add_line("AAABBA");
        container.add_line("AAABBA");
        container.add_line("ABBAAA");
        container.add_line("ABBAAA");
        container.add_line("AAAAAA");
        container.after_all_line();

        assert_eq!(container.star2(), "368".to_string());
    }
}
