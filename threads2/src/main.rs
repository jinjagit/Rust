// A very naive use of a function to start and stop a thread

use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    thread(1);

    for i in 1..20 {
        println!("hi number {} from the main thread!", i);

        if i > 15 {
            //thread(0);
        }
        thread::sleep(Duration::from_millis(20));
    } 
}

fn thread (bool: u8) {
    let (tx, rx) = mpsc::channel();

    if bool == 1 {
        thread::spawn(move || {
            for i in 1..1000 {
                println!("Working...{}", i);
                //thread::sleep(Duration::from_millis(5));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating.");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        });
    } else {
        let _ = tx.send(());
    }
}