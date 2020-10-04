// My, very naive, implementation of the threadpool in lib.rs (from tutorial)
// Finds Sum(sine(a), sine(a + 1), , sine(a + 2) ... sine(a + n)), where a = 1, n = 50000000
// Does not seem to calculate sum significantly faster with more threads == something not quite right here.

use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn main() {
    //test();

    let thread_counter = Arc::new(AtomicU32::new(0));
    let counter_clone = thread_counter.clone();

    let sum: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
    let sum_clone = sum.clone();

    let num_threads = get_num_threads();

    let start_time = Instant::now();

    let pool = ThreadPool::new(num_threads as u8);

    let foo = move || {
        let operation_block = counter_clone.fetch_add(1, Ordering::SeqCst);
        let mut sum = sum_clone.lock().unwrap();

        let default_iters_per_thread = 50000000 / num_threads;

        let start = default_iters_per_thread * operation_block;

        let mut result: f64 = 0.0;

        if operation_block == num_threads - 1 {
            for i in start + 1..50000001 {
                result = result + (i as f64).sin();
            }

            *sum += result;
        } else {
            for i in start + 1..(default_iters_per_thread * (operation_block + 1)) + 1 {
                result = result + (i as f64).sin();
            }

            *sum += result;
        }
    };

    for _ in 0..num_threads {
        pool.execute(foo.clone());
    }

    let mut completed_threads_count = thread_counter.load(Ordering::SeqCst);

    // Loop while threads not completed. Seems naive and probably inefficient
    while completed_threads_count < num_threads {
        std::thread::sleep(std::time::Duration::from_millis(25));
        completed_threads_count = thread_counter.load(Ordering::SeqCst);
    }

    
    println!("sum = {:.6}", sum.lock().unwrap());
    let elapsed = start_time.elapsed();
    println!("{:?} ms", elapsed.as_millis());
}

fn get_input() -> String {
    let mut s = String::new();
    print!("How many threads? Enter an integer from 1 to 12: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

fn get_num_threads() -> u32 {
    let mut i: u32 = 127;

    while i > 12 || i == 0 {
        let input = get_input();
        // println!("You typed: {}", input);

        i = match input.parse::<u32>() {
            Ok(i) => i,
            Err(_) => {
                println!("Error! Try again");
                127
            }
        };
    }

    i
}

// fn test() {
//     let mut result: f64 = 0.0;

//     for i in 1..50000001 {
//         result = result + (i as f64).sin();
//     }

//     println!("check: {:.6}", result);
// }
