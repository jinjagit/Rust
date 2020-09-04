#![allow(unused)]

use std::time::Instant;

fn main() {
    // roughly twice as fast as Vec<u64> for same process: 28ms for release
    let v: Vec<u32> = (1..100000001).collect();

    let start = Instant::now();
    let sum: u64 = v.iter().map(|x| *x as u64).sum();

    println!("{}", sum);

    let elapsed = start.elapsed();
    println!("{:?} ms", elapsed.as_millis());
}
    