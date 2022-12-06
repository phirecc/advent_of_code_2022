use std::{io::BufRead, collections::{HashMap, hash_map::Entry}};

use anyhow::Result;

fn find_first_marker(s: &str, length: usize) -> Option<usize> {
    let mut seen: HashMap<char, i64> = HashMap::new();
    let chars: Vec<_> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        seen.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        if i < length-1 {
            continue;
        }

        if seen.len() == length {
            return Some(i);
        }
        if let Entry::Occupied(mut o) = seen.entry(chars[i-(length-1)]) {
            let x = o.get_mut();
            *x -= 1;
            if *x == 0 {
                o.remove_entry();
            }
        }
    }
    None
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let line = input.lines().next().unwrap()?;
    Ok(vec![
       find_first_marker(&line, 4).unwrap()+1,
       find_first_marker(&line, 14).unwrap()+1,
    ])
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
        assert_eq!(solve(include_bytes!("../../data/day06_example.txt").as_slice()).unwrap(), [7, 19]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day06_input.txt").as_slice()).unwrap(), [1080, 3645]);
    }
}
