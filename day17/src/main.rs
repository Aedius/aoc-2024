use helper::{InputReader, Solver};
use Bytes::*;
use Instruction::*;

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "4,6,3,5,6,3,5,2,1,0".to_string(),
        result1: Some("7,3,0,5,7,1,4,0,5".to_string()),
        example2: None,
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day17");
}

#[derive(Copy, Clone, Debug)]
enum Bytes {
    B000,
    B001,
    B010,
    B011,
    B100,
    B101,
    B110,
    B111,
}

impl Into<u8> for Bytes {
    fn into(self) -> u8 {
        match self {
            B000 => 0,
            B001 => 1,
            B010 => 2,
            B011 => 3,
            B100 => 4,
            B101 => 5,
            B110 => 6,
            B111 => 7,
        }
    }
}
impl Into<usize> for Bytes {
    fn into(self) -> usize {
        match self {
            B000 => 0,
            B001 => 1,
            B010 => 2,
            B011 => 3,
            B100 => 4,
            B101 => 5,
            B110 => 6,
            B111 => 7,
        }
    }
}

impl From<usize> for Bytes {
    fn from(value: usize) -> Self {
        match value {
            0 => B000,
            1 => B001,
            2 => B010,
            3 => B011,
            4 => B100,
            5 => B101,
            6 => B110,
            7 => B111,
            _ => {
                panic!("value was too big")
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<Bytes> for Instruction {
    fn from(value: Bytes) -> Self {
        match value {
            B000 => Adv,
            B001 => Bxl,
            B010 => Bst,
            B011 => Jnz,
            B100 => Bxc,
            B101 => Out,
            B110 => Bdv,
            B111 => Cdv,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Computer {
    instructions: Vec<Bytes>,
    pointer: usize,
    a: usize,
    b: usize,
    c: usize,
    output: Vec<usize>,
}

impl Computer {
    fn compute(&mut self) {
        let max_time = 100;
        let mut n = 0;
        while self.pointer < self.instructions.len() && n < max_time {
            self.operate(
                self.instructions[self.pointer].into(),
                self.instructions[self.pointer + 1].into(),
            );
            n += 1;
        }
    }

    fn operate(&mut self, instruction: Instruction, operand: Bytes) {
        match instruction {
            Adv => {
                let val = self.get_combo_value(operand);
                self.a = self.a / (2_usize.pow(val as u32))
            }
            Bxl => {
                let val: usize = operand.into();
                self.b = self.b ^ val
            }
            Bst => {
                let val = self.get_combo_value(operand);
                self.b = val % 8
            }
            Jnz => match self.a {
                0 => {}
                _ => {
                    self.pointer = operand.into();
                    return;
                }
            },
            Bxc => self.b = self.b ^ self.c,
            Out => {
                let val = self.get_combo_value(operand);
                self.output.push(val % 8)
            }
            Bdv => {
                let val = self.get_combo_value(operand);
                self.b = self.a / (2_usize.pow(val as u32))
            }
            Cdv => {
                let val = self.get_combo_value(operand);
                self.c = self.a / (2_usize.pow(val as u32))
            }
        }
        self.pointer += 2;
    }

    fn get_combo_value(&self, b: Bytes) -> usize {
        match b {
            B000 => 0,
            B001 => 1,
            B010 => 2,
            B011 => 3,
            B100 => self.a,
            B101 => self.b,
            B110 => self.c,
            B111 => {
                panic!("operand 7 is reserved")
            }
        }
    }
}

#[derive(Debug, Default)]
struct Container {
    computer: Computer,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if line.starts_with("Register A: ") {
            self.computer.a = line[12..].parse::<usize>().unwrap()
        }
        if line.starts_with("Register B: ") {
            self.computer.b = line[12..].parse::<usize>().unwrap()
        }
        if line.starts_with("Register C: ") {
            self.computer.c = line[12..].parse::<usize>().unwrap()
        }
        if line.starts_with("Program: ") {
            self.computer.instructions = line[9..]
                .split(',')
                .into_iter()
                .map(|v| v.parse::<usize>().unwrap().into())
                .collect()
        }
    }

    fn star1(&self) -> String {
        let mut computer = self.computer.clone();
        computer.compute();
        let list: Vec<String> = computer.output.into_iter().map(|v| v.to_string()).collect();
        list.join(",")
    }

    fn star2(&self) -> String {
        if self.computer.instructions.len() < 10 {
            return "nope".to_string();
        }

        let to_find: Vec<usize> = self
            .computer
            .clone()
            .instructions
            .into_iter()
            .map(|a| a.into())
            .collect();

        for a in 8616000000..10_000_000_000 {
            if a % 1_000_000 == 0 {
                println!("{a}");
            }

            let mut computer = self.computer.clone();
            computer.a = a;
            computer.compute();

            if computer.output == to_find {
                return a.to_string();
            }
        }
        "failed".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut computer = Computer {
            instructions: vec![2.into(), 6.into()],
            pointer: 0,
            a: 0,
            b: 0,
            c: 9,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.b, 1);
    }
    #[test]
    fn test_2() {
        let mut computer = Computer {
            instructions: vec![5.into(), 0.into(), 5.into(), 1.into(), 5.into(), 4.into()],
            pointer: 0,
            a: 10,
            b: 0,
            c: 0,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.output, vec![0, 1, 2]);
    }
    #[test]
    fn test_3() {
        let mut computer = Computer {
            instructions: vec![0.into(), 1.into(), 5.into(), 4.into(), 3.into(), 0.into()],
            pointer: 0,
            a: 2024,
            b: 0,
            c: 0,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);
    }
    #[test]
    fn test_4() {
        let mut computer = Computer {
            instructions: vec![1.into(), 7.into()],
            pointer: 0,
            a: 0,
            b: 29,
            c: 0,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.b, 26);
    }
    #[test]
    fn test_5() {
        let mut computer = Computer {
            instructions: vec![4.into(), 0.into()],
            pointer: 0,
            a: 0,
            b: 2024,
            c: 43690,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.b, 44354);
    }
    #[test]
    fn test_star2() {
        let mut computer = Computer {
            instructions: vec![0.into(), 3.into(), 5.into(), 4.into(), 3.into(), 0.into()],
            pointer: 0,
            a: 117440,
            b: 0,
            c: 0,
            output: vec![],
        };
        computer.compute();

        assert_eq!(computer.output, vec![0, 3, 5, 4, 3, 0]);
    }
}
