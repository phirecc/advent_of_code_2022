use std::{io::BufRead, collections::HashSet, cmp::max};

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let l = line?;
        let buf: Vec<i32> = l.chars().map(|c| c as i32 - '0' as i32).collect();
        map.push(buf);
    }
    let mut visible = HashSet::new();
    for x in 0..map[0].len() {
        visible.insert((x, 0));
        visible.insert((x, map.len()-1));
        let mut max = map[0][x];
        for y in 1..map.len() {
            if map[y][x] > max {
                visible.insert((x,y));
                max = map[y][x];
            }
        }
        let mut max = map[map.len()-1][x];
        for y in (0..map.len()-1).rev() {
            if map[y][x] > max {
                visible.insert((x,y));
                max = map[y][x];
            }
        }
    }
    for y in 0..map.len() {
        visible.insert((0, y));
        visible.insert((map[0].len()-1, y));
        let mut max = map[y][0];
        for x in 1..map[0].len() {
            if map[y][x] > max {
                visible.insert((x,y));
                max = map[y][x];
            }
        }
        let mut max = map[y][map.len()-1];
        for x in (0..map[0].len()-1).rev() {
            if map[y][x] > max {
                visible.insert((x,y));
                max = map[y][x];
            }
        }
    }

    // part2
    let mut max_score = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let mut score_left = 0;
            for bx in (0..x as i32).rev() {
                score_left += 1;
                if map[y][bx as usize] >= map[y][x] {
                    break;
                }
            }
            let mut score_right = 0;
            for bx in x+1..map[0].len() {
                score_right += 1;
                if map[y][bx] >= map[y][x] {
                    break;
                }
            }
            let mut score_up = 0;
            for by in (0..y as i32).rev() {
                score_up += 1;
                if map[by as usize][x] >= map[y][x] {
                    break;
                }
            }
            let mut score_down = 0;
            for by in y+1..map.len() {
                score_down += 1;
                if map[by][x] >= map[y][x] {
                    break;
                }
            }
            max_score = max(max_score, score_left * score_right * score_up * score_down);
        }
    }

    Ok(vec![visible.len(), max_score])
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
        assert_eq!(solve(include_bytes!("../../data/day08_example.txt").as_slice()).unwrap(), [21, 8]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day08_input.txt").as_slice()).unwrap(), [1560, 252000]);
    }
}
