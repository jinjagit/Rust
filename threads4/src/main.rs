use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::mpsc::channel;


fn long_running_function(x: u64, shared: &AtomicBool) -> Option<u64> {
    for i in 0..x {
        if shared.load(Ordering::Relaxed) {
            return None
        }
        println!("{} at step {}", x, i);
        thread::sleep(Duration::from_secs(x));
    }
    shared.swap(true, Ordering::Relaxed);
    Some(x * 2)
}

fn main() {
    let (tx, rx) = channel();
    let shared = Arc::new(AtomicBool::new(false));
    for i in 1..80 {
        let tx1 = tx.clone();
        let shared1 = Arc::clone(&shared);
        thread::spawn(move || {
            let result = long_running_function(i, &shared1);
            tx1.send(result).unwrap()
        });
    }
    if let Ok(Some(n)) = rx.recv() {
        println!("{}", n);
    }
}
