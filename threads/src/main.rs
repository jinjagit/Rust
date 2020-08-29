// We can fix the problem of the spawned thread not getting to run, or not
// getting to run completely, by saving the return value of thread::spawn in a
// variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an
// owned value that, when we call the join method on it, will wait for its
// thread to finish. Listing 16-2 shows how to use the JoinHandle of the thread
// we created in Listing 16-1 and call join to make sure the spawned thread
// finishes before main exits:

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Because join is called before the for loop in main, it will block the
    // main thread until the spawned thread terminates
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
