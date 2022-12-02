use std::io::BufRead;

use anyhow::Result;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn points(&self) -> i64  {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn wins(&self, other: &Self) -> i64 {
        if *self == *other {
            return 3;
        }
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            _ => 0,
        }
    }
}

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut score = 0;
    for line in input.lines() {
        let v: Vec<Shape> = line?.split(' ').map(|x| {
            match x {
                "A"|"X" => Shape::Rock,
                "B"|"Y" => Shape::Paper,
                "C"|"Z" => Shape::Scissors,
                _ => panic!("Invalid shape {}", x)
            }
        }).collect();
        score += v[1].points() + v[1].wins(&v[0]);
    }
    Ok(vec![score])
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
        assert_eq!(solve(include_bytes!("../../data/day02_example.txt").as_slice()).unwrap(), [15]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day02_input.txt").as_slice()).unwrap(), [13526]);
    }
}
