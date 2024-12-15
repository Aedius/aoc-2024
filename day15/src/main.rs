use crate::Move::*;
use crate::Tile::*;
use helper::{InputReader, Solver};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "10092".to_string(),
        result1: Some("1406392".to_string()),
        example2: Some("9021".to_string()),
        result2: Some("1429013".to_string()),
        kind: Default::default(),
    };

    solver.solve("day15");
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
struct Container {
    map: Map,
    moves: Vec<Move>,
    map_with: usize,
    robot: Position,
}

type Map = Vec<Vec<Tile>>;

trait Mapped {
    fn display(&self) -> String;
}

impl Mapped for Map {
    fn display(&self) -> String {
        let mut chars = Vec::new();

        for rows in self {
            for tile in rows {
                match tile {
                    Wall => chars.push('#'),
                    Box => chars.push('O'),
                    Robot => chars.push('@'),
                    Empty => chars.push('.'),
                    BoxLeft => chars.push('['),
                    BoxRight => chars.push(']'),
                }
            }
            chars.push('\n')
        }

        chars.into_iter().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Wall,
            'O' => Box,
            '[' => BoxLeft,
            ']' => BoxRight,
            '@' => Robot,
            '.' => Empty,
            _ => {
                panic!("wrong tile kind")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Left,
            '>' => Right,
            '^' => Up,
            'v' => Down,
            _ => {
                panic!("wrong move")
            }
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        self.moves.reverse();
        for (y, rows) in self.map.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                if tile == &Robot {
                    self.robot = Position { x, y };
                    return;
                }
            }
        }
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.map_with == 0 {
            self.map_with = line.len()
        }
        if line.len() > self.map_with {
            let mut moves: Vec<Move> = line.chars().map(|c| c.into()).collect();
            self.moves.append(&mut moves)
        } else {
            let row: Vec<Tile> = line.chars().map(|c| c.into()).collect();

            self.map.push(row);
        }
    }

    fn star1(&self) -> String {
        let mut container = self.clone();
        while container.tick().is_some() {}

        container.gps().to_string()
    }

    fn star2(&self) -> String {
        let mut container = self.enlarge();
        while container.tick().is_some() {}

        println!("{}", container.map.display());

        container.gps().to_string()
    }
}

impl Container {
    fn enlarge(&self) -> Self {
        let mut map = Vec::new();

        for rows in &self.map {
            let mut row = Vec::new();
            for tile in rows {
                match tile {
                    Wall => {
                        row.push(Wall);
                        row.push(Wall);
                    }
                    Box => {
                        row.push(BoxLeft);
                        row.push(BoxRight);
                    }
                    BoxLeft => {}
                    BoxRight => {}
                    Robot => {
                        row.push(Robot);
                        row.push(Empty);
                    }
                    Empty => {
                        row.push(Empty);
                        row.push(Empty);
                    }
                }
            }
            map.push(row);
        }

        Container {
            map,
            moves: self.moves.clone(),
            map_with: self.map_with * 2,
            robot: Position {
                x: self.robot.x * 2,
                y: self.robot.y,
            },
        }
    }
    fn tick(&mut self) -> Option<()> {
        let before = self.map.clone();
        let mut current_move: Move = Left;
        let mut current_moving = None;
        let res = match self.moves.pop() {
            None => None,
            Some(m) => {
                current_move = m.clone();
                let moving = self.get_movable((self.robot.clone(), Robot), &m);
                current_moving = moving.clone();
                match moving {
                    None => {}
                    Some(moving) => {
                        for (p, _) in &moving {
                            self.map[p.y][p.x] = Empty;
                        }

                        for (p, tile) in moving {
                            let next = p.next(&m);
                            self.map[next.y][next.x] = tile;
                            if tile == Robot {
                                self.robot = next;
                            }
                        }
                    }
                }

                Some(())
            }
        };

        if !self.check_big() {
            println!("current move {current_move:?}");
            println!("{:?}", current_moving);
            println!("{}", before.display());
            println!("{}", self.map.display());
            panic!("OUPSI");
        }

        res
    }

    fn check_big(&self) -> bool {
        for rows in &self.map {
            for w in rows.windows(2) {
                if w[0] == BoxLeft && w[1] != BoxRight {
                    return false;
                }
            }
        }
        true
    }

    fn get_movable(&self, current: (Position, Tile), m: &Move) -> Option<Vec<(Position, Tile)>> {
        let mut movable = vec![];

        let next = current.0.next(m);
        movable.push(current.clone());
        let next_tile = self.map[next.y][next.x];
        match next_tile {
            Wall => None,
            Box => self
                .get_movable((next.clone(), next_tile), m)
                .map(|mut moving| {
                    movable.append(&mut moving);
                    movable
                }),
            Robot => {
                panic!("another robots ?")
            }
            Empty => Some(movable),
            BoxLeft => match m {
                Up => self
                    .get_big_box_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Right => self
                    .get_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Down => self
                    .get_big_box_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Left => self
                    .get_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
            },
            BoxRight => match m {
                Up => self
                    .get_big_box_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Right => self
                    .get_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Down => self
                    .get_big_box_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
                Left => self
                    .get_movable((next.clone(), next_tile), m)
                    .map(|mut moving| {
                        movable.append(&mut moving);
                        movable
                    }),
            },
        }
    }

    fn get_big_box_movable(
        &self,
        next: (Position, Tile),
        m: &Move,
    ) -> Option<Vec<(Position, Tile)>> {
        let mut movable: Vec<(Position, Tile)> = vec![];

        let direct = self.get_movable(next.clone(), m);

        let next_pos = next.0.clone();

        let other = self.get_movable(
            (
                Position {
                    x: if next.1 == BoxLeft {
                        next_pos.x + 1
                    } else {
                        next_pos.x - 1
                    },
                    y: next_pos.y,
                },
                if next.1 == BoxLeft { BoxRight } else { BoxLeft },
            ),
            m,
        );

        match (direct, other) {
            (Some(mut direct), Some(mut right)) => {
                movable.append(&mut direct);
                movable.append(&mut right);
                Some(movable)
            }
            (_, _) => None,
        }
    }

    fn gps(&self) -> usize {
        let mut res = 0;
        for (y, rows) in self.map.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                match tile {
                    Wall => {}
                    Box => res += 100 * y + x,
                    BoxLeft => res += 100 * y + x,
                    BoxRight => {}
                    Robot => {}
                    Empty => {}
                }
            }
        }

        res
    }
}

impl Position {
    fn next(&self, m: &Move) -> Self {
        match m {
            Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_robot() {
        let mut container = Container::from_str(
            r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#,
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move <:"
        );

        println!("{}", container.map.display());
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move ^:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move ^:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move >:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move >:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
            )
            .map
            .display(),
            "Move >:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move v:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move v:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move <:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move v:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move >:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move >:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move v:"
        );

        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move <:"
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#
            )
            .map
            .display(),
            "Move <:"
        );

        assert_eq!(container.gps(), 2028, "gps")
    }

    #[test]
    fn big_room() {
        let small = Container::from_str(
            r#"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#,
        );

        let mut container = small.enlarge();

        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############"#
            )
            .map
            .display()
        );
        container.tick();

        println!("{}", container.map.display());
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############"#
            )
            .map
            .display()
        );
        container.tick();
        assert_eq!(
            container.map.display(),
            Container::from_str(
                r#"
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############"#
            )
            .map
            .display()
        );
    }
}
