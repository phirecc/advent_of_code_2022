#![feature(iter_next_chunk)]
use std::io::BufRead;
use std::ops::{Add, Mul};

use anyhow::{Result, bail};

#[derive(Default)]
struct Monkey {
    items: Vec<usize>,
    inspect: Option<Box<dyn Fn(usize) -> usize>>,
    next_monkey: Option<Box<dyn Fn(usize) -> usize>>,
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut monkeys = Vec::new();
    let mut cur_monkey: Monkey = Default::default();
    let mut lines = input.lines().peekable();
    let mut common_div = 1;
    while let Some(line) = lines.next() {
        let l = line?;
        let t = l.trim();
        if t.starts_with("Starting items") {
            let items: Vec<usize> = t.rsplit_once(": ").unwrap().1.split(", ").map(|x| x.parse().unwrap()).collect();
            cur_monkey.items = items;
        } else if t.starts_with("Operation") {
            let formula: Vec<&str> = t.rsplit_once(" = ").unwrap().1.split(' ').collect();
            let op: Box<dyn Fn(usize, usize) -> usize> = match formula[1] {
                "+" => Box::new(usize::add),
                "*" => Box::new(usize::mul),
                _ => bail!("Invalid operation: {}", formula[1])
            };
            let f: Box<dyn Fn(usize) -> usize> = {
                let x = formula[2].parse::<usize>();
                if let Ok(x) = x {
                    Box::new(move |z| op(z, x))
                } else {
                    Box::new(move |z| op(z, z))
                }
            };
            cur_monkey.inspect = Some(f);
        } else if t.starts_with("Test") {
            let x: usize = t.rsplit_once(' ').unwrap().1.parse().unwrap();
            common_div *= x;
            let variants = lines.next_chunk::<2>().unwrap().map(|l| l.unwrap().trim().rsplit_once(' ').unwrap().1.parse::<usize>().unwrap());
            cur_monkey.next_monkey = Some(Box::new(move |z| if z % x == 0 {
                variants[0]
            } else {
                variants[1]
            }))
        } else if t.is_empty() {
            monkeys.push(cur_monkey);
            cur_monkey = Default::default();
        }
    }
    monkeys.push(cur_monkey);
    let mut values2 = Vec::new();
    for m in &monkeys {
        values2.push(m.items.clone());
    }
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut m = std::mem::take(&mut monkeys[i]);
            for item in &m.items {
                let new_value = m.inspect.as_ref().unwrap()(*item) / 3;
                let next = m.next_monkey.as_ref().unwrap()(new_value);
                monkeys[next].items.push(new_value);
                inspections[i] += 1;
            }
            m.items.clear();
            monkeys[i] = m;
        }
    }
    inspections.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspections[0] * inspections[1];

    // part 2
    let mut inspections2 = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &monkeys[i];
            let v = std::mem::take(&mut values2[i]);
            for item in v {
                let new_value = m.inspect.as_ref().unwrap()(item) % common_div;
                let next = m.next_monkey.as_ref().unwrap()(new_value);
                values2[next].push(new_value);
                inspections2[i] += 1;
            }
        }
    }
    inspections2.sort_by(|a, b| b.cmp(a));
    let monkey_business2 = inspections2[0] * inspections2[1];

    Ok(vec![monkey_business, monkey_business2])
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
        assert_eq!(solve(include_bytes!("../../data/day11_example.txt").as_slice()).unwrap(), [10605, 2713310158]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day11_input.txt").as_slice()).unwrap(), [110220, 19457438264]);
    }
}
