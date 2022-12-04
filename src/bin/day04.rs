use std::{io::BufRead};

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut res1 = 0;
    for line in input.lines() {
        let l = line?;
        let mut pairs = l.split(',')
            .map(|s| {
                let x: Vec<i64> = s.split('-').map(|a| a.parse().unwrap()).collect();
                (x[0], x[1])
            });
        let a = pairs.next().unwrap();
        let b = pairs.next().unwrap();
        if (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1) {
            res1 += 1;
        }
    }
    Ok(vec![res1])
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
        assert_eq!(solve(include_bytes!("../../data/day04_example.txt").as_slice()).unwrap(), [2]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day04_input.txt").as_slice()).unwrap(), [534]);
    }
}
