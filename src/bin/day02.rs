use std::io::BufRead;

use anyhow::Result;

#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> i64  {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn eval(&self, other: &Self) -> i64 {
        if *self == *other {
            3
        } else if *other == self.winning_opponent() {
            0
        } else {
            6
        }
    }
    fn winning_opponent(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
    fn losing_opponent(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut score1 = 0;
    let mut score2 = 0;
    for line in input.lines() {
        let l = line?;
        let sp: Vec<&str> = l.split(' ').collect();
        let v: Vec<Shape> = sp.iter().map(|x| {
            match *x {
                "A"|"X" => Shape::Rock,
                "B"|"Y" => Shape::Paper,
                "C"|"Z" => Shape::Scissors,
                _ => panic!("Invalid shape {}", x)
            }
        }).collect();
        score1 += v[1].value() + v[1].eval(&v[0]);
        let shape2 = match sp[1] {
            "X" => v[0].losing_opponent(),
            "Y" => v[0],
            "Z" => v[0].winning_opponent(),
            _ => panic!("Invalid outcome {}", sp[1])
        };
        score2 += shape2.eval(&v[0]) + shape2.value();
    }
    Ok(vec![score1, score2])
}

fn main() -> Result<()> {
    for (i, s) in solve(std::io::stdin().lock())?.iter().enumerate() {
        println!("part {}: {}", i+1, s);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(include_bytes!("../../data/day02_example.txt").as_slice()).unwrap(), [15, 12]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day02_input.txt").as_slice()).unwrap(), [13526, 14204]);
    }
}
