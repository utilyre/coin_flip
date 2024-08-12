use std::collections::BTreeMap;

use rand::prelude::*;

const NUM_TRIES: usize = 1_000_000;

fn main() {
    let mut map: BTreeMap<usize, u32> = BTreeMap::new();

    let mut rng = rand::thread_rng();
    for _ in 0..NUM_TRIES {
        let mut num = 0;
        loop {
            num += 1;
            if rng.gen() {
                *map.entry(num).or_default() += 1;
                break;
            }
        }
    }

    println!("total = {}", NUM_TRIES);
    println!();
    println!(
        "{:^10} | {:^10} | {:^10} | {:^10}",
        "num", "count", "expected", "off"
    );
    println!("{:-^11}+{:-^12}+{:-^12}+{:-^11}", "", "", "", "");

    let mut probability = 1.0;
    for (&num, &count) in map.iter() {
        probability *= 0.5;

        println!(
            "{:<10} | {:<10} | %{:<9.3} | {:<10}",
            num,
            count,
            100.0 * probability,
            count as f64 - (NUM_TRIES as f64 * probability).floor()
        );
    }
}
