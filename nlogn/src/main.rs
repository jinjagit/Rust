// find n for n log n = 1000000 (& for base 2 log)

// Iterate f(n) == 1000000 / (log n) until result unchanging == approximation of n

fn main() {
    let mut estimate: f64 = 100.0;
    let mut count: u32 = 0;

    loop {
        count += 1;
        let result: f64 = 1000000.0 / estimate.log2();

        if result == estimate {
            break;
        }

        estimate = result;
    }

    println!("result: {}", estimate);
    println!("iterations: {}", count);
}
