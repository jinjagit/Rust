fn main() {
    sine_series(10);
}

fn sine_series(n: i32) -> f32 {
    let mut counter = 1.0;
    let stop = (n + 1) as f32;
    let mut result: f32 = 0.0;

    while counter != stop {
        result = result + counter.sin(); 
        println!("{}", counter);

        counter += 1.0;
    }

    result
}
