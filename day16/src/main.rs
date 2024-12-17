use std::cmp::{ Ordering};
use crate::Direction::*;
use crate::Tile::*;
use helper::{InputReader, Solver};
use std::collections::HashMap;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "11048".to_string(),
        result1: Some("109516".to_string()),
        example2: Some("64".to_string()),
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day16");
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Empty,
    End,
    Start,
    Path(Direction, usize),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Wall,
            '.' => Empty,
            'S' => Start,
            'E' => End,
            _ => panic!("wrong char {value}"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Position{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x){
            Ordering::Less => {Ordering::Less}
            Ordering::Equal => {
                self.y.cmp(&other.y)
            }
            Ordering::Greater => {Ordering::Greater}
        }
    }
}

impl Position {
    fn get_next(&self, direction: Direction) -> Self {
        match direction {
            North => Position {
                x: self.x,
                y: self.y - 1,
            },
            East => Position {
                x: self.x + 1,
                y: self.y,
            },
            South => Position {
                x: self.x,
                y: self.y + 1,
            },
            West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Container {
    map: HashMap<Position, Tile>,
    start: Position,
    height: usize,
    width: usize,
    min_score: usize,
    snake_by_score:HashMap<usize, Vec<Vec<SnakePart>>>
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        self.min_score = usize::MAX
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.width = line.len();
        for (x, c) in line.chars().enumerate() {
            let position = Position { x, y: self.height };
            let mut tile: Tile = c.into();
            if tile == Start {
                self.start = position;
                tile = Path(East, 0);
            }

            self.map.insert(position, tile);
        }

        self.height += 1;
    }

    fn star1(&self) -> String {
        let mut container = self.clone();

        let mut head = vec![vec![
            SnakePart {
                position: self.start,
                score: 0,
                direction: East,
            }]];

        while !head.is_empty(){
            head = container.find_next(head, false);
        }

        container.min_score.to_string()
    }

    fn star2(&self) -> String {
        let mut container = self.clone();

        let mut head = vec![vec![
            SnakePart {
                position: self.start,
                score: 0,
                direction: East,
            }]];

        while !head.is_empty(){
            head = container.find_next(head, true);
        }

        let snakes= container.snake_by_score.get(&container.min_score).unwrap();

        let mut positions : Vec<Position>= snakes.iter().flatten().into_iter().map(|sp| sp.position).collect();

        positions.sort();
        positions.dedup();

        positions.len().to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct SnakePart {
    position: Position,
    score: usize,
    direction: Direction,
}

impl SnakePart{
    fn compute_score(&self, direction: Direction) -> usize {
        self.score + if self.direction == direction { 1 } else { 1001 }
    }
}

impl Container {
    fn find_next(&mut self, snakes: Vec<Vec<SnakePart>>, all_path: bool) -> Vec<Vec<SnakePart>> {
        let mut output = Vec::new();
        for snake in snakes {
            let head = snake.last().unwrap();

            for direction in vec![North, East, South, West] {
                let next_position = head.position.get_next(direction);
                let next = self.map.get_mut(&next_position).unwrap();
                let new_value = match next {
                    Wall => { None }
                    Start => { None }
                    Empty => {
                        Some(SnakePart {
                            position: next_position,
                            score: head.compute_score(direction),
                            direction,
                        })
                    }
                    End => {
                        let new_score = head.score + 1;
                        dbg!(new_score);
                        if new_score <= self.min_score {
                            self.min_score = new_score;
                            let entry = self.snake_by_score.entry(new_score).or_default();
                            entry.push(snake.clone());
                        }
                        None
                    }
                    Path(_direction, path_score) => {

                        let new_snake_part = SnakePart {
                            position: next_position,
                            score: head.compute_score(direction),
                            direction,
                        };

                        // if !all_path{
                            if head.compute_score(direction) <= *path_score {
                                Some(new_snake_part)
                            } else {
                                None
                            }
                        // }else {
                        //     if snake.contains(&new_snake_part){
                        //        None
                        //     }else{
                        //         Some(new_snake_part)
                        //     }
                        // }
                    }
                };
                if let Some(new_head) = new_value {
                    *next = Path(North, new_head.score);
                    let mut new_snake = snake.clone();
                    new_snake.push(new_head);
                    output.push(new_snake);
                }
            }
        }

        output
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let container = Container::from_str(
            r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#,
        );

        assert_eq!(container.star1(), 7036.to_string());
        assert_eq!(container.star2(), 45.to_string());
    }
}
