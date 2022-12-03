use std::{io::BufRead, collections::{HashMap, HashSet}};

use anyhow::Result;

fn get_priority(c: char) -> i64 {
    if c.is_ascii_lowercase() {
        c as i64 - 'a' as i64 + 1
    } else {
        c as i64 - 'A' as i64 + 27
    }
}

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut type_count: HashMap<char, i64> = HashMap::new();
    for (lnum, line) in input.lines().enumerate() {
        let mut seen = HashSet::new();
        let mut priorities = HashSet::new();
        let l = line?;
        let first_half = &l[..l.len()/2];
        let second_half = &l[l.len()/2..];
        first_half.chars().for_each(|c| { seen.insert(c); });
        for c in second_half.chars() {
            if seen.contains(&c) {
                priorities.insert(get_priority(c));
            }
        }
        sum1 += priorities.iter().sum::<i64>();

        // part 2: also insert second half into seen
        second_half.chars().for_each(|c| { seen.insert(c); });
        for c in seen {
            *type_count.entry(c).or_default() += 1;
        }

        if (lnum+1) % 3 == 0 {
            for (k, v) in &type_count {
                if *v == 3 {
                    sum2 += get_priority(*k);
                    break;
                }
            }
            type_count.clear();
        }
    }
    Ok(vec![sum1, sum2])
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
        assert_eq!(solve(include_bytes!("../../data/day03_example.txt").as_slice()).unwrap(), [157, 70]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day03_input.txt").as_slice()).unwrap(), [7597, 2607]);
    }
}
