use std::{io::BufRead, collections::HashSet};

use anyhow::{Result, bail};

fn get_delta(parent: (i32, i32), child: (i32, i32)) -> (i32, i32) {
    let mut delta = (0i32, 0i32);
    if (parent.0 - child.0).abs() > 1 {
        delta.0 += parent.0-child.0;
        if (parent.1 - child.1).abs() >= 1 {
            delta.1 += parent.1-child.1;
        }
    } else if (parent.1 - child.1).abs() > 1 {
        delta.1 += parent.1-child.1;
        if (parent.0 - child.0).abs() >= 1 {
            delta.0 += parent.0-child.0;
        }
    }
    (delta.0.clamp(-1, 1), delta.1.clamp(-1, 1))
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    let mut rope = [(0i32, 0i32); 10];
    visited.insert((0,0));
    visited2.insert((0,0));
    for line in input.lines() {
        let l = line?;
        let mut s = l.split(' ');
        let dir = s.next().unwrap();
        let steps: i32 = s.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            let head = &mut rope[9];
            match dir {
                "R" => *head = (head.0 + 1, head.1),
                "L" => *head = (head.0 - 1, head.1),
                "D" => *head = (head.0, head.1 + 1),
                "U" => *head = (head.0, head.1 - 1),
                _ => bail!("Invalid direction: {}", dir)
            }
            for i in (0..rope.len()-1).rev() {
                let d = get_delta(rope[i+1], rope[i]);
                rope[i] = (rope[i].0 + d.0, rope[i].1 + d.1);
            }
            visited.insert(rope[8]);
            visited2.insert(rope[0]);
        }
    }
    Ok(vec![visited.len(), visited2.len()])
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
        assert_eq!(solve(include_bytes!("../../data/day09_example.txt").as_slice()).unwrap(), [13, 1]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day09_input.txt").as_slice()).unwrap(), [5710, 2259]);
    }
}
