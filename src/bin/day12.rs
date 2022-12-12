use std::{io::BufRead, collections::{VecDeque, HashMap}};

use anyhow::{Result, bail};

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut heightmap = Vec::new();
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        let l = line?;
        let v: Vec<char> = l.chars().collect();
        if let Some(pos) = v.iter().position(|x| *x == 'S') {
            start = Some((pos as i32, i as i32)); 
        }
        if let Some(pos) = v.iter().position(|x| *x == 'E') {
            end = Some((pos as i32, i as i32)); 
        }
        heightmap.push(v);
    }
    let start = start.unwrap();
    let end = end.unwrap();

    heightmap[start.1 as usize][start.0 as usize] = 'a';
    heightmap[end.1 as usize][end.0 as usize] = 'z';

    let mut q = VecDeque::new();
    q.push_back(start);
    let mut layers = HashMap::new();
    layers.insert(start, 0);
    let mut dist = 0;
    'outer: while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for x in [(0,1), (0,-1), (1,0), (-1,0)] {
            let v = (cur.0 + x.0, cur.1 + x.1);
            if v.0 < 0 || v.1 < 0 || v.0 as usize >= heightmap[0].len() || v.1 as usize >= heightmap.len() {
                continue
            }
            if heightmap[v.1 as usize][v.0 as usize] as i32 - heightmap[cur.1 as usize][cur.0 as usize] as i32 > 1 {
                continue;
            }
            if !layers.contains_key(&v) {
                q.push_back(v);
                layers.insert(v, layers.get(&cur).unwrap()+1);
                if v == end {
                    dist = *layers.get(&v).unwrap();
                    break 'outer;
                }
            }
        }
    }
    Ok(vec![dist])
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
        assert_eq!(solve(include_bytes!("../../data/day12_example.txt").as_slice()).unwrap(), [31]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day12_input.txt").as_slice()).unwrap(), [481]);
    }
}
