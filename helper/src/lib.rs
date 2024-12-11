use easy_reader::EasyReader;
use std::marker::PhantomData;
use std::{fs::File, io::Error};

pub trait InputReader {
    fn on_start(&mut self) {}
    fn after_all_line(&mut self) {}

    fn read(&mut self, path: &str) -> Result<(), Error> {
        self.on_start();
        let file = File::open(path)?;

        let mut reader = EasyReader::new(file)?;

        while let Some(line) = reader.next_line()? {
            self.add_line(line.as_str());
        }
        self.after_all_line();

        Ok(())
    }

    fn add_line(&mut self, line: &str);

    fn star1(&self) -> String;
    fn star2(&self) -> String;
}

pub struct Solver<T> {
    pub example1: String,
    pub result1: Option<String>,
    pub example2: Option<String>,
    pub result2: Option<String>,
    pub kind: PhantomData<T>,
}

impl<T: Default + InputReader> Solver<T> {
    pub fn solve(self, path: &str) {
        let mut container_example = T::default();
        container_example
            .read(&format!("./{}/test.txt", path))
            .unwrap();

        assert_eq!(container_example.star1(), self.example1, "Example star 1");

        let mut container_result = T::default();
        container_result
            .read(&format!("./{}/input.txt", path))
            .unwrap();

        let star1 = container_result.star1();
        println!("Star 1 : {}", star1);
        if let Some(result1) = self.result1 {
            assert_eq!(star1, result1, "Result star 1");
        }

        if let Some(example2) = self.example2 {
            assert_eq!(container_example.star2(), example2, "Example star 2");
        } else {
            println!("Example 2 : {}", container_example.star2());
        }
        let star2 = container_result.star2();
        println!("Star 2 : {}", star2);

        if let Some(result2) = self.result2 {
            assert_eq!(star2, result2, "Result star 2");
        }
    }
}
