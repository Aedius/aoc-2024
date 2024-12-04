use enum_iterator::{all, Sequence};
use helper::{InputReader, Solver};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "18".to_string(),
        result1: Some("2406".to_string()),
        example2: Some("9".to_string()),
        result2: Some("1807".to_string()),
        kind: Default::default(),
    };

    solver.solve("day04");
}

#[derive(Debug, Default)]
struct Container {
    data: Vec<Vec<char>>,
}

#[derive(Sequence, Copy, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    fn opp(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::TopLeft => Direction::BottomRight,
            Direction::TopRight => Direction::BottomLeft,
            Direction::BottomLeft => Direction::TopRight,
            Direction::BottomRight => Direction::TopLeft,
        }
    }
    fn rot(&self) -> Direction {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
            Direction::TopLeft => Direction::TopRight,
            Direction::TopRight => Direction::BottomRight,
            Direction::BottomRight => Direction::BottomLeft,
            Direction::BottomLeft => Direction::TopLeft,
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        self.data.push(line.chars().collect())
    }

    fn star1(&self) -> String {
        let mut result = 0;

        let len = self.data.first().unwrap().len();

        for i in 0..self.data.len() {
            for j in 0..len {
                if self.data[i][j] == 'X' {
                    for dir in all::<Direction>() {
                        if self.is_xmas(i.try_into().unwrap(), j.try_into().unwrap(), dir) {
                            result += 1;
                        }
                    }
                }
            }
        }

        result.to_string()
    }

    fn star2(&self) -> String {
        let mut result = 0;

        let len = self.data.first().unwrap().len();

        for i in 0..self.data.len() {
            for j in 0..len {
                if self.data[i][j] == 'A' {
                    for dir in all::<Direction>() {
                        if self.is_x_mas(i.try_into().unwrap(), j.try_into().unwrap(), dir) {
                            result += 1;
                        }
                    }
                }
            }
        }

        result.to_string()
    }
}

impl Container {
    fn is_xmas(&self, i: isize, j: isize, dir: Direction) -> bool {
        if self.get(next((i, j), dir)) == Some(&'M')
            && self.get(next(next((i, j), dir), dir)) == Some(&'A')
            && self.get(next(next(next((i, j), dir), dir), dir)) == Some(&'S')
        {
            return true;
        }

        false
    }

    fn is_x_mas(&self, i: isize, j: isize, dir: Direction) -> bool {
        match dir {
            Direction::Top => {
                return false;
            }
            Direction::Bottom => {
                return false;
            }
            Direction::Left => {
                return false;
            }
            Direction::Right => {
                return false;
            }
            _ => {}
        }

        let diag = dir.rot();
        if self.get(next((i, j), dir)) == Some(&'M')
            && self.get(next((i, j), dir.opp())) == Some(&'S')
            && self.get(next((i, j), diag)) == Some(&'M')
            && self.get(next((i, j), diag.opp())) == Some(&'S')
        {
            return true;
        }

        false
    }

    fn get(&self, (i, j): (isize, isize)) -> Option<&char> {
        if i < 0 {
            return None;
        }
        if j < 0 {
            return None;
        }

        let i: usize = i.try_into().unwrap();
        let j: usize = j.try_into().unwrap();

        match self.data.get(i) {
            None => None,
            Some(line) => line.get(j),
        }
    }
}

fn next((i, j): (isize, isize), dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Top => (i - 1, j),
        Direction::Bottom => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
        Direction::TopLeft => next(next((i, j), Direction::Left), Direction::Top),
        Direction::TopRight => next(next((i, j), Direction::Right), Direction::Top),
        Direction::BottomLeft => next(next((i, j), Direction::Left), Direction::Bottom),
        Direction::BottomRight => next(next((i, j), Direction::Right), Direction::Bottom),
    }
}
