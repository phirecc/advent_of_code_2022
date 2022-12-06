use std::{io::BufRead, collections::{HashMap, hash_map::Entry}};

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let line = input.lines().next().unwrap()?;
    let mut seen: HashMap<char, i64> = HashMap::new();
    let chars: Vec<_> = line.chars().collect();
    let mut res1: i64 = -1;
    for (i, c) in chars.iter().enumerate() {
        seen.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        if i < 3 {
            continue;
        }

        if seen.len() == 4 {
            res1 = i as i64 + 1;
            break;
        }
        if let Entry::Occupied(mut o) = seen.entry(chars[i-3]) {
            let x = o.get_mut();
            *x -= 1;
            if *x == 0 {
                o.remove_entry();
            }
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
        assert_eq!(solve(include_bytes!("../../data/day06_example.txt").as_slice()).unwrap(), [7]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day06_input.txt").as_slice()).unwrap(), [1080]);
    }
}
