// My, very naive, implementation of using a threadpool to share work in parallel
// Uses the threadpool in lib.rs (from https://www.youtube.com/watch?v=2mwwYbBRJSo)
// Finds Sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50000000

extern crate num_cpus;

use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn main() {
    // count logical cores this process could try to use
    let num_cores = num_cpus::get();
    println!("number of logical cores available: {}\n", num_cores);
    println!("Find sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50000000\n");

  loop {
      let thread_counter = Arc::new(AtomicU32::new(0));
      let counter_clone = thread_counter.clone();

      // This var seems unnecessary. ?Find way to use counter_clone for this?
      let completed_counter = Arc::new(AtomicU32::new(0));
      let completed_clone = completed_counter.clone();

      let sum: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
      let sum_clone = sum.clone();

      let num_threads = get_num_threads();

      let start_time = Instant::now();

      let pool = ThreadPool::new(num_threads as u8);

      let foo = move || {
          let operation_block = counter_clone.fetch_add(1, Ordering::SeqCst);
          let default_iters_per_thread = 50000000 / num_threads;
          let start = default_iters_per_thread * operation_block;
          let mut result: f64 = 0.0;

          if operation_block == num_threads - 1 {
              for i in start + 1..50000001 {
                  result = result + (i as f64).sin();
              }
          } else {
              for i in start + 1..(default_iters_per_thread * (operation_block + 1)) + 1 {
                  result = result + (i as f64).sin();
              }
          }

          let mut sum = sum_clone.lock().unwrap();
          *sum += result;
          completed_clone.fetch_add(1, Ordering::SeqCst);
      };

      for _ in 0..num_threads {
          pool.execute(foo.clone());
      }

      let mut completed_threads_count = completed_counter.load(Ordering::SeqCst);

      // Loop while threads not completed.
      while completed_threads_count < num_threads {
          std::thread::sleep(std::time::Duration::from_millis(5));
          completed_threads_count = completed_counter.load(Ordering::SeqCst);
      }

      
      let elapsed = start_time.elapsed();
      println!("sum = {:.6}", sum.lock().unwrap());
      println!("{:?} ms\n", elapsed.as_millis());
    }
}

fn get_input() -> String {
    let mut s = String::new();
    print!("Split calculation across how many threads?\nEnter an integer from 1 to 12: ");
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

    while i > 12 {
        let input = get_input();
        // println!("You typed: {}", input);

        i = match input.parse::<u32>() {
            Ok(i) => if i > 0 && i < 13 {
              i
            } else {
              println!("\nERROR! Try again");
              127
            }
            Err(_) => {
                println!("\nERROR! Try again");
                127
            }
        };
    }

    i
}
