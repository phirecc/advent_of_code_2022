#![feature(iter_array_chunks)]
use std::io::BufRead;

use anyhow::{Result, bail};

#[derive(Debug, PartialEq, Clone)]
enum PacketMember {
    Raw(i32),
    Array(Vec<PacketMember>),
}

#[derive(PartialEq, Debug)]
enum ComparisonResult {
    Continue,
    RightOrder,
    WrongOrder
}

fn compare_raw(l: i32, r: i32) -> ComparisonResult {
    if l < r {
        ComparisonResult::RightOrder
    } else if l > r {
        ComparisonResult::WrongOrder
    } else {
        ComparisonResult::Continue
    }
}

fn compare_members(left: &PacketMember, right: &PacketMember) -> ComparisonResult {
    match &left {
        PacketMember::Raw(l) => {
            match right {
                PacketMember::Raw(r) => {
                    compare_raw(*l, *r)
                },
                PacketMember::Array(_) => {
                    compare_members(&PacketMember::Array(vec![PacketMember::Raw(*l)]), right)
                },
            }
        },
        PacketMember::Array(l) => {
            match right {
                PacketMember::Raw(r) => {
                    compare_members(left, &PacketMember::Array(vec![PacketMember::Raw(*r)]))
                },
                PacketMember::Array(r) => {
                    let m = std::cmp::min(l.len(), r.len());
                    for i in 0..m {
                        let res = compare_members(&l[i], &r[i]);
                        if res != ComparisonResult::Continue {
                            return res;
                        }
                    }
                    compare_raw(l.len() as i32, r.len() as i32)
                },
            }
        },
    }
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut part1 = 0;
    let mut part2 = Vec::new();
    for (i, pair) in input.lines().map(|s| s.unwrap()).filter(|s| !s.is_empty()).array_chunks::<2>().enumerate() {
        let mut pair = pair.map(|p| {
            let mut cur = Box::new(PacketMember::Array(Vec::new()));
            let mut parents = Vec::new();
            let mut num = None;
            for c in p.chars() {
                if [']', ','].contains(&c) {
                    if let PacketMember::Array(ref mut v) = *cur {
                        if let Some(x) = num {
                            v.push(PacketMember::Raw(x));
                        }
                    }
                    num = None;
                }
                match c {
                    '[' => {
                        parents.push(cur);
                        cur = Box::new(PacketMember::Array(Vec::new()));
                    }
                    ',' => {}
                    ']' => {
                        let l = parents.pop();
                        if let Some(mut x) = l {
                            if let PacketMember::Array(v) = &mut *x {
                                v.push(*cur);
                                cur = x;
                            }
                        }
                    }
                    '0'..='9' => {
                        if num.is_none() {
                            num = Some(0);
                        }
                        num = Some(num.unwrap()*10 + c as i32 - '0' as i32);
                    },
                    _ => {
                        bail!("Invalid char: {}", c);
                    }
                };
            }
            Ok(cur)
        }).map(|x| {
            if let PacketMember::Array(a) = *x.unwrap() {
                Ok(a)
            } else {
                bail!("topmost member not an array");
            }
        }).map(|x| x.unwrap());
        let res = compare_members(&pair[0][0], &pair[1][0]);
        if res == ComparisonResult::RightOrder {
            part1 += i+1;
        }
        part2.append(&mut pair[0]);
        part2.append(&mut pair[1]);
    }
    let m1 = PacketMember::Array(vec![PacketMember::Array(vec![PacketMember::Raw(2)])]);
    let m2 = PacketMember::Array(vec![PacketMember::Array(vec![PacketMember::Raw(6)])]);
    part2.push(m1.clone());
    part2.push(m2.clone());
    part2.sort_by(|a, b| match compare_members(a, b){
        ComparisonResult::Continue => std::cmp::Ordering::Equal,
        ComparisonResult::RightOrder => std::cmp::Ordering::Less,
        ComparisonResult::WrongOrder => std::cmp::Ordering::Greater,
    });
    let mut sol2 = 1;
    for (i, x) in part2.iter().enumerate() { 
        if *x == m1 || *x == m2 {
            sol2 *= i+1;
        }
    }
    Ok(vec![
       part1,
       sol2
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
        assert_eq!(solve(include_bytes!("../../data/day13_example.txt").as_slice()).unwrap(), [13, 140]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day13_input.txt").as_slice()).unwrap(), [5366, 23391]);
    }
}
