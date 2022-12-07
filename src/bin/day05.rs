use std::{io::BufRead, collections::VecDeque};

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<String>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = input.lines();
    // Parse stacks
    while let Some(line) = lines.next() {
        let l = line?;
        if l.starts_with(" 1") {
            break;
        }
        let mut idx = 0;
        let mut cur: &str = &l;
        while !cur.is_empty() {
            if idx >= stacks.len() {
                stacks.push(VecDeque::new());
            }
            let (chunk, mut rest) = cur.split_at(3);
            if !rest.is_empty() {
                rest = &rest[1..];
            }
            let ch = chunk[1..chunk.len()-1].bytes().next().unwrap() as char;
            if !ch.is_whitespace() {
                stacks[idx].push_front(ch as char);
            }
            cur = rest;
            idx += 1;
        }
    }
    let mut stacks2 = stacks.clone();
    lines.next();
    // Parse and apply moves
    for line in lines {
        let l = line?;
        let mut parts = l.split(' ');
        let n = parts.nth(1).unwrap().parse().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..n {
            let x = stacks[from].pop_back().unwrap();
            stacks[to].push_back(x);
        }
        // part 2
        let r = stacks2[from].len()-n..;
        let mut dest = std::mem::take(&mut stacks2[to]);
        dest.extend(stacks2[from].drain(r));
        stacks2[to] = dest;
    }
    let sol = |x: Vec<VecDeque<_>>| x.iter().map(|s| s.back().unwrap()).collect();
    Ok(vec![sol(stacks), sol(stacks2)])
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
        assert_eq!(solve(include_bytes!("../../data/day05_example.txt").as_slice()).unwrap(), ["CMZ", "MCD"]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day05_input.txt").as_slice()).unwrap(), ["PTWLTDSJV", "WZMFVGGZP"]);
    }
}
