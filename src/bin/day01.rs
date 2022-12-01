use anyhow::Result;

fn main() -> Result<()> {
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
    for line in std::io::stdin().lines() {
        let l = line?;
        if l.is_empty() {
            update(cur);
            cur = 0;
        } else {
            cur += l.parse::<i64>()?;
        }
    }
    update(cur);
    println!("part 1: {}", top3[2]);
    println!("part 2: {}", top3.iter().sum::<i64>());
    Ok(())
}
