use std::time::Instant;

fn main() {
    let start = Instant::now();

    let x: f32 = sine_series(10000000);
    println!("x = {}", x);

    let elapsed = start.elapsed();
    println!("{:?}", elapsed.as_millis());
}

fn sine_series(n: i32) -> f32 {
    let mut counter = 1;
    let stop = n + 1;
    let mut result: f32 = 0.0;

    while counter != stop {
        result = result + (counter as f32).sin();
        counter += 1;
    }

    result
}

// => 122 - 127 ms to run on my PC (as release build)
// takes about 196 ms in Firefox as wasm (on same PC) = 1.57 times slower
// but often waaaay slower in other browsers, but usually still significantly
// faster than pure JS benchmark in respective browser(s).