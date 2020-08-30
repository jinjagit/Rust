use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // The send method returns a Result<T, E> type, so if the receiving end has
    // already been dropped and there’s nowhere to send a value, the send
    // operation will return an error.
    thread::spawn(move || {
        let val = String::from("hello from a thread!");
        tx.send(val).unwrap();
    });


    // recv will block the main thread’s execution and wait until a value is
    // sent down the channel. try_recv method doesn’t block, but will instead
    // return a Result<T, E> immediately (so would need a loop to check for rx)
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    send_with_pauses();
}

fn send_with_pauses() {
    let (tx, rx) = mpsc::channel();

    // before we create the first spawned thread, we call clone on the sending
    // end of the channel. This will give us a new sending handle we can pass to
    // the first spawned thread. We pass the original sending end of the channel
    // to a second spawned thread. This gives us two threads, each sending
    // different messages to the receiving end of the channel.
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("pausing"),
            String::from("thread"),
        ];

        // When the channel is closed, iteration will end.
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
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
