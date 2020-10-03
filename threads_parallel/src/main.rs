// Proof of concept:  parallel threads aree faster than one!

use std::time::Instant;
use std::thread;
use std::sync::mpsc;

fn main() {
    let start = Instant::now();

    let result = f64_sines(10000000);

    let now = Instant::now();
    let elapsed: u64 = now.duration_since(start).as_millis() as u64;

    println!("result from main: {}, in {} ms", result, elapsed);


    // This runs just as fast as main, but only when built as release!
    // (270ms vs. 267ms)
    let start2 = Instant::now();

    let result2 = f64_sine_thread(10000000);

    let now2 = Instant::now();
    let elapsed2: u64 = now2.duration_since(start2).as_millis() as u64;

    println!("result from thread: {}, in {} ms", result2, elapsed2);


    // Now lets call 2 threads in parallel, each with half number of iterations:

    let start3 = Instant::now();

    let result_a = f64_sine_two_threads(10000000);

    let now3 = Instant::now();
    let elapsed3: u64 = now3.duration_since(start3).as_millis() as u64;

    println!("result from 2 parallel threads: {}, in {} ms", result_a, elapsed3);
}

fn f64_sines(n: u64) -> f64 {
    let mut counter = 1;
    let stop = n + 1;
    let mut result: f64 = 0.0;

    while counter != stop {
        result = result + (counter as f64).sin();
        counter += 1;
    }

    result
}

fn f64_sine_thread(n: u64) -> f64 {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut result: f64 = 0.0;
        for i in 1..n + 1 {
            result = result + (i as f64).sin();
        }

        tx.send(result).unwrap();
    });

    let result = rx.recv().unwrap();
    result
}

fn f64_sine_two_threads(n: u64) -> f64 {
    let (tx, rx) = mpsc::channel();

    // This thread does first half of n iterations
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let mut result: f64 = 0.0;
        for i in 1..(n / 2) + 1 {
            result = result + (i as f64).sin();
        }

        tx1.send(result).unwrap();
    });

    // This thread does second half of n iterations
    thread::spawn(move || {
        let mut result: f64 = 0.0;
        for i in (n / 2) + 1..n + 1 {
            result = result + (i as f64).sin();
        }

        tx.send(result).unwrap();
    });

    // sum the 2 results from the 2 threads
    let mut result: f64 = 0.0;
    for received in rx { result = result + received; }

    result
}