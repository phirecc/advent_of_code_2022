use std::{io::BufRead, collections::HashMap};

use anyhow::{Result, bail};

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut map = HashMap::new();
    let mut max_y = 0;
    for line in input.lines() {
        let l = line?;
        let sp: Vec<_> = l.split(" -> ").map(|s| {
            let mut x = s.split(',').map(|s| s.parse::<i32>().unwrap());
            (x.next().unwrap(), x.next().unwrap())
        }).collect();
        for i in 1..sp.len() {
            assert!(sp[i].0 == sp[i-1].0 || sp[i].1 == sp[i-1].1);
            let x1 = std::cmp::min(sp[i].0, sp[i-1].0);
            let x2 = std::cmp::max(sp[i].0, sp[i-1].0);
            let y1 = std::cmp::min(sp[i].1, sp[i-1].1);
            let y2 = std::cmp::max(sp[i].1, sp[i-1].1);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    map.insert((x, y), 0);
                    max_y = std::cmp::max(max_y, y);
                }
            }
        }
    }
    let mut part1 = 0;
    let mut map2 = map.clone();
    let start = (500, 0);
    'outer: loop {
        let mut pos = start;
        loop {
            let candidates = [(pos.0, pos.1+1), (pos.0-1, pos.1+1), (pos.0+1, pos.1+1)];
            let mut new_pos = None;
            for c in candidates {
                if !map.contains_key(&c) {
                    new_pos = Some(c);
                    break;
                }
            }
            if let Some(p) = new_pos {
                if pos.1 > max_y {
                    break 'outer;
                }
                pos = p;
            } else {
                part1 += 1;
                map.insert(pos, 1);
                break;
            }
        }
    }
    // part 2
    let mut part2 = 0;
    'outer: loop {
        let mut pos = start;
        loop {
            let candidates = [(pos.0, pos.1+1), (pos.0-1, pos.1+1), (pos.0+1, pos.1+1)];
            let mut new_pos = None;
            for c in candidates {
                if !map2.contains_key(&c) && c.1 != max_y+2 {
                    new_pos = Some(c);
                    break;
                }
            }
            if let Some(p) = new_pos {
                pos = p;
            } else {
                part2 += 1;
                map2.insert(pos, 1);
                if pos == start {
                    break 'outer;
                }
                break;
            }
        }
    }
    Ok(vec![
       part1,
       part2
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
        assert_eq!(solve(include_bytes!("../../data/day14_example.txt").as_slice()).unwrap(), [24, 93]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day14_input.txt").as_slice()).unwrap(), [768, 26686]);
    }
}
