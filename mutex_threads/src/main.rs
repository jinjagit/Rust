// Mutexes have a reputation for being difficult to use because you have to
// remember two rules:
    // You must attempt to acquire the lock before using the data.
    // When you’re done with the data that the mutex guards, you must unlock the
    // data so other threads can acquire the lock.

// Mutex<T> comes with the risk of creating deadlocks. These occur when an
// operation needs to lock two resources and two threads have each acquired one
// of the locks, causing them to wait for each other forever.

// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
// The a stands for atomic, meaning it’s an atomically reference counted type.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
