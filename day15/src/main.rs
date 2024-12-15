use helper::{InputReader, Solver};
use crate::Move::*;
use crate::Tile::*;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "10092".to_string(),
        result1: None,
        example2: None,
        result2: None,
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
                    Wall => { chars.push('#') }
                    Box => { chars.push('O') }
                    Robot => { chars.push('@') }
                    Empty => { chars.push('.') }
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
    Robot,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => { Wall }
            'O' => { Box }
            '@' => { Robot }
            '.' => { Empty }
            _ => {
                todo!("wrong tile")
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
            '<' => { Left }
            '>' => { Right }
            '^' => { Up }
            'v' => { Down }
            _ => { todo!("wrong move") }
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
                    self.robot = Position {
                        x,
                        y,
                    };
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
            let mut moves: Vec<Move> = line.chars().into_iter().map(|c| c.into()).collect();
            self.moves.append(&mut moves)
        } else {
            let row: Vec<Tile> = line.chars().into_iter().map(|c| c.into()).collect();

            self.map.push(row);
        }
    }

    fn star1(&self) -> String {

        let mut container = self.clone();
        while container.tick().is_some(){

        }

        container.gps().to_string()
    }

    fn star2(&self) -> String {
        todo!("star2")
    }
}

impl Container {
    fn tick(&mut self) -> Option<()> {
        match self.moves.pop() {
            None => { None }
            Some(m) => {
                let mut current = self.robot.clone();
                let mut can_move = vec![current.clone()];


                loop {
                    let next = current.next(&m);
                    match self.map[next.y][next.x] {
                        Wall => {
                            can_move = Vec::new();
                            break;
                        }
                        Box => {
                            can_move.push(next.clone());
                        }
                        Robot => {
                            println!("{}", self.map.display());

                            todo!("another robots ?")
                        }
                        Empty => {
                            break;
                        }
                    }
                    current = next;
                }


                if !can_move.is_empty() {
                    while let Some(p) = can_move.pop() {
                        let next = p.next(&m);
                        self.map[next.y][next.x] = self.map[p.y][p.x];
                    }
                    self.map[self.robot.y][self.robot.x] = Empty;
                    self.robot = self.robot.next(&m);
                }


                Some(())
            }
        }
    }


    fn gps(&self) -> usize {
        let mut res = 0;
        for (y, rows) in self.map.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                if tile == &Box {
                    res += 100 * y + x
                }
            }
        }

        res
    }
}

impl Position {
    fn next(&self, m: &Move) -> Self {
        match m {
            Up => {
                Position {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            Right => {
                Position {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            Down => {
                Position {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            Left => {
                Position {
                    x: self.x - 1,
                    y: self.y,
                }
            }
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

<^^>>>vv<v>>v<<"#);

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move <:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move ^:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move ^:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move >:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move >:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        ).map.display(), "Move >:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move v:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move v:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move <:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move v:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move >:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move >:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########"#
        ).map.display(), "Move v:");

        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move <:");
        container.tick();
        assert_eq!(container.map.display(), Container::from_str(
            r#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#
        ).map.display(), "Move <:");

        assert_eq!(container.gps(), 2028, "gps")
    }
}

