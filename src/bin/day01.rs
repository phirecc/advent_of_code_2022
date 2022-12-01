use anyhow::Result;

fn main() -> Result<()> {
    let mut cur = 0;
    let mut best = 0;
    for line in std::io::stdin().lines() {
        let l = &line?;
        if l.is_empty() {
            best = std::cmp::max(cur, best);
            cur = 0;
        } else {
            let x: i64 = l.parse()?;
            cur += x;
        }
    }
    println!("part 1: {}", best);
    Ok(())
}
