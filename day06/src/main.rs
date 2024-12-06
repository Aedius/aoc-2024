
use helper::{InputReader, Solver};
fn main() {
    let solver: Solver<Container> = Solver {
        example1: "41".to_string(),
        result1: None,
        example2: None,
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day06");
}

#[derive(Debug, Default)]
struct Container {
    map: Vec<Vec<Tile>>,
    start: Gard,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Obstacle,
}
#[derive(Debug, Default)]
enum Direction {
    #[default]
    North,
    Est,
    South,
    West,
}

impl Direction{
    fn next(&self)->Self{
        match self {
            Direction::North => {Direction::Est}
            Direction::Est => {Direction::South}
            Direction::South => {Direction::West}
            Direction::West => {Direction::North}
        }
    }
 }

#[derive(Debug, Default)]
struct Gard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        let mut row = Vec::new();

        for c in line.chars() {
            match c {
                '.' => row.push(Tile::Empty),
                '#' => row.push(Tile::Obstacle),
                '^' => {
                    self.start = Gard {
                        direction: Direction::North,
                        x: self.map.len(),
                        y: row.len(),
                    };
                    row.push(Tile::Empty);
                }
                _ => {
                    todo!("{c}");
                }
            }
        }

        self.map.push(row);
    }

    fn star1(&self) -> String {

        dbg!(&self);
        self.screen(None);

        todo!("star1")
    }

    fn star2(&self) -> String {
        todo!("star2")
    }
}

impl Container{
    fn screen(&self, path: Option<Vec<(usize, usize)>>){
        for x in 0..self.map.len(){
            for y in 0.. self.map[0].len(){
                let s = if x == self.start.x && y == self.start.y {
                    "0"
                }else {
                    match self.map[x][y] {
                        Tile::Empty => { "." }
                        Tile::Obstacle => { "X" }
                    }
                };
                print!("{s}");
            }
            println!()
        }
    }
}
