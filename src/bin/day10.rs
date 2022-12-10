use std::io::BufRead;

use anyhow::{Result, bail};

fn solve<T: BufRead>(input: T) -> Result<Vec<i32>> {
    let mut cycle = 0;
    let mut x = 1;
    let mut part1 = 0;
    for line in input.lines() {
        let l = line?;
        let mut s = l.split_whitespace();
        let ins = s.next().unwrap();
        let (dur, new_x) = match ins {
            "addx" => {
                let op: i32 = s.next().unwrap().parse().unwrap();
                (2, x+op)
            },
            "noop" => (1, x),
            _ => bail!("Invalid instruction: {}", ins)
        };
        for _ in 0..dur {
            cycle += 1;
            if cycle % 40 == 20 {
                part1 += cycle*x;
            }
        }
        x = new_x;
    }
    Ok(vec![part1])
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
        assert_eq!(solve(include_bytes!("../../data/day10_example.txt").as_slice()).unwrap(), [13140]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day10_input.txt").as_slice()).unwrap(), [15360]);
    }
}
