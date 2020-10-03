// Threads intro. tutorial: https://www.youtube.com/watch?v=JXDkdaGEuVU

// To create a thread:
// call thread::spawn function
// pass a closure containing code we want to run into that new thread

// To prevent spawned thread death when main thread ends:
// Save the spawned thread return value in a variable:
// spawn returns a 'JoinHandle'. When we call 'join' on it,
// it waits for the thread to finish. In some cases, this
// blocks main until the thread finishes.

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // This blocks main thread until spawned thread finishes
    handle.join().unwrap();


    // Using 'move' to take ownership

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); //Error: value used here after move (to spawned thread)

    handle.join().unwrap();


    // Using a channel to send a message
    // 'mpsc' = 'multi-producer, single consumer'

    let (tx, rx) = mpsc::channel();

    // spawn takes ownership of tx, using 'move'
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Note, we cannot use 'val' in this thread now,
        // as we have given ownership to the reciever.
    });

    let received = rx.recv().unwrap();
    println!("Main got: {}", received);


    // We can do this the other way too,
    // sending message from main to spawned thread:

    let (tx, rx) = mpsc::channel();

    let val = String::from("hi");
        tx.send(val).unwrap();

    // spawn takes ownership of rx, using 'move'
    let handle = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("Thread got: {}", received);
    });  

    // Note need to block main theread until spawned thread completes
    handle.join().unwrap();


    // Cloning the transmiter to feed multiple messages to ther receiver:

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("sent by"),
            String::from("thread2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}