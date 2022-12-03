use std::io::BufRead;

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut sum = 0;
    for line in input.lines() {
        let mut seen = std::collections::HashSet::new();
        let mut priorities = std::collections::HashSet::new();
        let l = line?;
        for (i, c) in l.chars().enumerate() {
            if i < l.len()/2 {
                seen.insert(c);
            } else {
                if seen.contains(&c) {
                    let x = if c.is_ascii_lowercase() {
                        c as i64 - 'a' as i64 + 1
                    } else {
                        c as i64 - 'A' as i64 + 27
                    };
                    priorities.insert(x);
                }
            }
        }
        sum += priorities.iter().sum::<i64>();
    }
    Ok(vec![sum])
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
        assert_eq!(solve(include_bytes!("../../data/day03_example.txt").as_slice()).unwrap(), [157]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day03_input.txt").as_slice()).unwrap(), [7597]);
    }
}
