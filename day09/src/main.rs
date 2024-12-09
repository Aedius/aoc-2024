use helper::{InputReader, Solver};
use std::fmt::{Debug, Display, Formatter};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "1928".to_string(),
        result1: Some("6332189866718".to_string()),
        example2: Some("2858".to_string()),
        result2: Some("6353648390778".to_string()),
        kind: Default::default(),
    };

    solver.solve("day09");
}

#[derive(Debug, Default)]
struct Container {
    disk_map: Vec<usize>,
    disk: Disk,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.disk_map = line
            .chars()
            .map(|c| (c.to_string()).parse::<usize>().unwrap())
            .collect();

        self.disk = Disk::new(&self.disk_map);
    }

    fn star1(&self) -> String {
        let mut disk = self.disk.clone();
        println!("{}", disk);
        disk.defragment_star1();
        println!("{}", disk);
        disk.checksum().to_string()
    }

    fn star2(&self) -> String {
        let mut disk = self.disk.clone();
        println!("{}", disk);
        disk.defragment_star2();
        println!("{}", disk);
        disk.checksum().to_string()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Data {
    Empty,
    File(usize),
}

#[derive(Debug, Clone)]
struct Slot {
    start: usize,
    length: usize,
}

#[derive(Default, Clone, Debug)]
struct Disk {
    data: Vec<Data>,
    empty: usize,
    empty_slots: Vec<Slot>,
    data_slots: Vec<(Slot, Data)>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.len() > 100 {
            return write!(f, "too big for display ! Length: {}", self.data.len());
        }
        for d in &self.data {
            write!(
                f,
                "{}",
                match d {
                    Data::Empty => {
                        ".".to_string()
                    }
                    Data::File(v) => {
                        v.to_string()
                    }
                }
            )?
        }
        write!(f, "")
    }
}

impl Disk {
    fn new(disk_map: &Vec<usize>) -> Self {
        let mut disk = Vec::new();
        let mut number = 0;
        let mut is_data = true;
        let mut empty = 0;
        let mut empty_slots = Vec::new();
        let mut data_slots = Vec::new();

        for c in disk_map {
            if is_data {
                data_slots.push((
                    Slot {
                        start: disk.len(),
                        length: *c,
                    },
                    Data::File(number),
                ));
                for _ in 0..*c {
                    disk.push(Data::File(number));
                }
                number += 1;
            } else {
                empty += *c;
                empty_slots.push(Slot {
                    start: disk.len(),
                    length: *c,
                });
                for _ in 0..*c {
                    disk.push(Data::Empty);
                }
            }

            is_data = !is_data;
        }
        Self {
            data: disk,
            empty,
            empty_slots,
            data_slots,
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (i, d) in self.data.iter().enumerate() {
            match d {
                Data::Empty => {}
                Data::File(u) => {
                    sum += i * *u;
                }
            }
        }
        sum
    }

    fn defragment_star1(&mut self) {
        let new_len = self.data.len() - self.empty;
        let mut remove: Vec<Data> = self
            .data
            .drain(new_len..)
            .filter(|d| d != &Data::Empty)
            .collect();

        for slot in &self.empty_slots {
            if slot.length > remove.len() {
                for (i, d) in remove.iter().rev().enumerate() {
                    self.data[slot.start + i] = d.clone()
                }
                return;
            }
            let remaining = remove.len() - slot.length;
            let add: Vec<Data> = remove.drain(remaining..).rev().collect();

            for (i, d) in add.iter().enumerate() {
                self.data[slot.start + i] = d.clone()
            }
        }
    }
    fn defragment_star2(&mut self) {
        while let Some((slot, data)) = self.data_slots.pop() {
            for (index, empty) in self.empty_slots.clone().iter().enumerate() {
                if empty.start > slot.start {
                    break;
                }

                if slot.length <= empty.length {
                    for i in 0..slot.length {
                        self.data[empty.start + i] = data.clone();
                        self.data[slot.start + i] = Data::Empty;
                    }
                    self.empty_slots[index] = Slot {
                        start: empty.start + slot.length,
                        length: empty.length - slot.length,
                    };
                    break;
                }
            }
        }
    }
}
