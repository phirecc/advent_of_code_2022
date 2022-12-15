use std::{io::BufRead, collections::HashSet};

use anyhow::Result;

fn solve<T: BufRead>(input: T, y: i32, y2: i32) -> Result<Vec<usize>> {
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
    let mut sensor_dist = Vec::new();
    for (sensor, beacon) in pairs {
        let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        sensor_dist.push((sensor, dist));
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
    // part2:
    // The missing beacon must be at the perimiter of a sensor. Walk around the perimiter of each
    // sensor, if the current square is not in the scan radius of any other sensor, the beacon is
    // at this square (because it is stated that there is only *one* possible square).
    let mut part2 = None;
    'outer: for (sensor, dist) in &sensor_dist {
        for dx in 0..=dist+1 {
            let dy = (dist+1) - dx;
            // each direction (signs)
            for (sx, sy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let x = sensor.0 + (dx*sx);
                let y = sensor.1 + (dy*sy);
                let r = 0..=y2;
                if !r.contains(&x) || !r.contains(&y) {
                    continue;
                }
                // check if it's in the scan radius of any sensor
                let mut found = true;
                for (s2, d2) in &sensor_dist {
                    if (s2.0 - x).abs() + (s2.1 - y).abs() <= *d2 {
                        found = false;
                        break;
                    }
                }
                if found {
                    part2 = Some(x as usize * 4_000_000 + y as usize);
                    break 'outer;
                }
            }
        }
    }
    Ok(vec![
       no_beacon.len(),
       part2.unwrap()
    ])
}

fn main() -> Result<()> {
    for (i, s) in solve(std::io::stdin().lock(), 10, 20)?.iter().enumerate() {
        println!("part {}: {}", i+1, s);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(include_bytes!("../../data/day15_example.txt").as_slice(), 10, 20).unwrap(), [26, 56000011]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day15_input.txt").as_slice(), 2000000, 4_000_000).unwrap(), [4876693, 11645454855041]);
    }
}
