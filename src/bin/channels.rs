use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel with a sender and receiver
    let (tx, rx) = mpsc::channel();

    // Create 10 sender threads
    let mut handles = vec![];

    for i in 0..10 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let messages = vec![
                format!("Hello from thread {}", i),
                format!("This is message 2 from thread {}", i),
                format!("This is message 3 from thread {}", i),
            ];

            for msg in messages {
                tx.send(msg).unwrap();
                println!("Thread {} sent a message", i);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    // Drop the original sender to allow the receiver to exit
    drop(tx);

    // Main thread receives messages
    for received in rx {
        println!("Received: {}", received);
    }

    // Wait for all sender threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
