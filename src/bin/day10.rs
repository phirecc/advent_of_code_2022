use std::io::BufRead;

use anyhow::{Result, bail};

fn solve<T: BufRead>(input: T) -> Result<(i32, String)> {
    let mut cycle = 0;
    let mut x = 1;
    let mut part1 = 0;
    let mut crt_buf = String::new();
    for line in input.lines() {
        let l = line?;
        let mut s = l.split_whitespace();
        let ins = s.next().unwrap();
        let (dur, new_x) = match ins {
            "addx" => {
                let op: i32 = s.next().unwrap().parse().unwrap();
                (2, x+op)
            },
            "noop" => (1, x),
            _ => bail!("Invalid instruction: {}", ins)
        };
        for _ in 0..dur {
            cycle += 1;
            if cycle % 40 == 20 {
                part1 += cycle*x;
            }
            if (x - (cycle-1) % 40).abs() <= 1 {
                crt_buf.push('#');
            } else {
                crt_buf.push('.');
            }
            if cycle % 40 == 0 {
                crt_buf.push('\n');
            }
        }
        x = new_x;
    }
    Ok((part1, crt_buf))
}

fn main() -> Result<()> {
    let sol = solve(std::io::stdin().lock())?;
    println!("part {}: {}", 1, sol.0);
    println!("part {}:\n{}", 2, sol.1);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let s =
"\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(solve(include_bytes!("../../data/day10_example.txt").as_slice()).unwrap(), (13140, s.into()));
    }
    #[test]
    fn input() {
        let s =
"\
###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.
";
        assert_eq!(solve(include_bytes!("../../data/day10_input.txt").as_slice()).unwrap(), (15360, s.into()));
    }
}
