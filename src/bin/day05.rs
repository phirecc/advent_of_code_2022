use std::io::BufRead;

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<String>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
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
                stacks.push(Vec::new());
            }
            let (chunk, mut rest) = cur.split_at(3);
            if !rest.is_empty() {
                rest = &rest[1..];
            }
            let ch = chunk[1..chunk.len()-1].bytes().next().unwrap() as char;
            if !ch.is_whitespace() {
                stacks[idx].insert(0, ch as char);
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
        let parts: Vec<&str> = l.split(' ').collect();
        let n = parts[1].parse().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;
        for _ in 0..n {
            let x = stacks[from].pop().unwrap();
            stacks[to].push(x);
        }
        // part 2
        let r = stacks2[from].len()-n..;
        let mut dest = std::mem::take(&mut stacks2[to]);
        dest.extend(stacks2[from].drain(r));
        stacks2[to] = dest;
    }
    let sol = |x: Vec<Vec<_>>| x.iter().map(|s| s.last().unwrap()).collect();
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
