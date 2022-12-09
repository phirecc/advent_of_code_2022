use std::{io::BufRead, collections::HashSet};

use anyhow::{Result, bail};

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut head = (0i32, 0i32); // (x, y)
    let mut tail = head;
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in input.lines() {
        let l = line?;
        let mut s = l.split(' ');
        let dir = s.next().unwrap();
        let steps: i32 = s.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            let prev = head;
            match dir {
                "R" => head = (head.0 + 1, head.1),
                "L" => head = (head.0 - 1, head.1),
                "D" => head = (head.0, head.1 + 1),
                "U" => head = (head.0, head.1 - 1),
                _ => bail!("Invalid direction: {}", dir)
            }
            if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
                tail = prev;
                visited.insert(tail);
            }
        }
    }
    Ok(vec![visited.len()])
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
        assert_eq!(solve(include_bytes!("../../data/day09_example.txt").as_slice()).unwrap(), [13]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day09_input.txt").as_slice()).unwrap(), [5710]);
    }
}
