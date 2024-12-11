use helper::{InputReader, Solver};

fn main() {
    let solver: Solver<Container> = Solver {
        example1: "55312".to_string(),
        result1: Some("197157".to_string()),
        example2: None,
        result2: None,
        kind: Default::default(),
    };

    solver.solve("day11");
}

#[derive(Debug, Default)]
struct Container {
    list: Vec<usize>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        self.list = line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
    }

    fn star1(&self) -> String {
        let mut res = self.list.clone();
        for i in 0..15 {
            res = blink(res);
            println!("{i} : {res:?}");
        }
        res.len().to_string()
    }

    fn star2(&self) -> String {
        todo!("");
        let mut res = self.list.clone();
        for i in 0..75 {
            res = blink(res);
            println!("{i} => {} // {:?}", res.len(), res.iter().max())
        }
        res.len().to_string()
    }
}

fn blink(origin: Vec<usize>) -> Vec<usize> {
    let mut destination = Vec::with_capacity(origin.len() * 2);

    
    for v in origin {
        match v.checked_ilog10() {
            None => {
                destination.push(1)
            }
            Some(n) => {
                if n % 2 == 1{
                    let power= n/2+1;
                    let tens :usize= (10_u32.pow( power)).try_into().unwrap();
                    destination.push(v / tens);
                    destination.push(v % tens)

                }else{
                    destination.push(v * 2024)}

            }
        }
    }


    destination
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut val =  blink(vec![32772608]);
        while ! val.contains(&32772608){
            val =  blink(val);
            dbg!(&val);
        }
    }
}
