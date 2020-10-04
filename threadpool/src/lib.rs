use std::sync::mpsc::{channel, Sender}; // {mpsc, Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
//use std::thread;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    tx: Sender<Box<dyn FnMut() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (tx, rx) = channel::<Box<dyn FnMut() + Send>>();
        let rx = Arc::new(Mutex::new(rx));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = rx.clone();
            let handle = std::thread::spawn(move || loop {
                let mut work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => break,
                };
                println!("start");
                work();
                println!("finish");
            });
        _handles.push(handle);
        }
 
        Self { _handles, tx }
    }

    // We need to use a generic type that implements a closure trait, like 'Fn()'
    pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
        self.tx.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let n = AtomicU32::new(0);
        let nref = Arc::new(n);
        // 1 line version of above 2 lines: let nref = Arc::new(AtomicU32::new(0));

        let pool = ThreadPool::new(10);
        let clone = nref.clone();
        let foo = move || {
            clone.fetch_add(1, Ordering::SeqCst);
        };
        pool.execute(foo.clone());
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(nref.load(Ordering::SeqCst), 2);
        // let c = pool.execute(|| sin(9.3)); // Can we get results back from closure like this? I doubt it!
    }
}

// Notes:

// JoinHandle: An owned permission to join on a thread (block on its termination).
// A JoinHandle detaches the associated thread when it is dropped, which means
// that there is no longer any handle to thread and no way to join on it.
