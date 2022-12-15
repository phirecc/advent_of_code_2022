use std::{io::BufRead, collections::HashSet};

use anyhow::Result;

fn solve<T: BufRead>(input: T, y: i32) -> Result<Vec<usize>> {
    let mut pairs = Vec::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let l = line?;
        let mut sp = l.split(|c| " ,:".contains(c)).filter(|s| s.contains('=')).map(|s| {
            let x = s.split('=').skip(1).next().unwrap();
            x.parse::<i32>().unwrap()
        });
        let sensor = (sp.next().unwrap(), sp.next().unwrap());
        let beacon = (sp.next().unwrap(), sp.next().unwrap());
        beacons.insert(beacon);
        pairs.push((sensor, beacon));
    }
    let mut no_beacon = HashSet::new();
    for (sensor, beacon) in pairs {
        let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        if (sensor.1 - dist .. sensor.1).contains(&y) || (sensor.1 .. sensor.1 + dist).contains(&y) {
            let delta = (sensor.1 - y).abs();
            let rl = dist - delta;
            for x in 0..=rl {
                for c in [(sensor.0 + x, y), (sensor.0 - x, y)] {
                    if !beacons.contains(&c) {
                        no_beacon.insert(c);
                    }
                }
            }
        }
    }
    Ok(vec![
       no_beacon.len()
    ])
}

fn main() -> Result<()> {
    for (i, s) in solve(std::io::stdin().lock(), 10)?.iter().enumerate() {
        println!("part {}: {}", i+1, s);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(include_bytes!("../../data/day15_example.txt").as_slice(), 10).unwrap(), [26]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day15_input.txt").as_slice(), 2000000).unwrap(), [4876693]);
    }
}
