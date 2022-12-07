use std::io::BufRead;

use anyhow::Result;

fn find_first_marker(s: &str, length: usize) -> Option<usize> {
    s.as_bytes().windows(length).position(|w| {
        w.iter()
            .map(|c| 1 << (*c as u8 - 'a' as u8))
            .fold(0u32, |acc, x| acc | x)
            .count_ones() == length as u32
    }).map(|x| x+length)
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let line = input.lines().next().unwrap()?;
    Ok(vec![
       find_first_marker(&line, 4).unwrap(),
       find_first_marker(&line, 14).unwrap(),
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
