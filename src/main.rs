use std::collections::BTreeMap;

use rand::prelude::*;

const NUM_TRIES: usize = 1_000_000;

fn main() {
    let mut map: BTreeMap<usize, u32> = BTreeMap::new();

    let mut rng = rand::thread_rng();
    for _ in 0..NUM_TRIES {
        let mut idx = 0;
        loop {
            idx += 1;
            if rng.gen() {
                *map.entry(idx).or_default() += 1;
                break;
            }
        }
    }

    for (idx, count) in map.iter() {
        println!("{:0>2} => {}", idx, count);
    }
}
