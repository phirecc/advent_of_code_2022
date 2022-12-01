use std::io::BufRead;

use anyhow::Result;

fn solve<T: BufRead>(input: T) -> Result<Vec<i64>> {
    let mut cur = 0;
    let mut top3 = [0; 3];
    let mut update = |cur| {
        for i in 0..top3.len() {
            if cur > top3[i] && (i == top3.len()-1 || cur < top3[i+1]) {
                for k in 0..i {
                    top3[k] = top3[k+1];
                }
                top3[i] = cur;
                break;
            }
        }
    };
    for line in input.lines() {
        let l = line?;
        if l.is_empty() {
            update(cur);
            cur = 0;
        } else {
            cur += l.parse::<i64>()?;
        }
    }
    update(cur);
    Ok(vec![top3[2], top3.iter().sum::<i64>()])
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
        assert_eq!(solve(include_bytes!("../../data/day01_example.txt").as_slice()).unwrap(), [24000, 45000]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day01_input.txt").as_slice()).unwrap(), [72478, 210367]);
    }
}
