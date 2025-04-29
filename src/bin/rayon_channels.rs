use rayon::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Print Rayon's thread pool configuration
    println!("Rayon thread pool size: {}", rayon::current_num_threads());
    println!("Number of logical CPUs: {}", num_cpus::get());

    // Create a channel with a sender and receiver
    let (tx, rx) = mpsc::channel();

    // Spawn the receiver in a separate thread
    let receiver_handle = thread::spawn(move || {
        println!("Receiver thread started");
        for received in rx {
            println!("Received: {}", received);
            thread::sleep(Duration::from_millis(50)); // Simulate some processing time
        }
        println!("Receiver thread finished");
    });

    // Create a vector of 10 items to process in parallel
    let items: Vec<usize> = (0..10).collect();

    println!("Starting parallel processing");
    // Process items in parallel using Rayon
    items.into_par_iter().for_each_with(tx, |tx, i| {
        let messages = vec![
            format!("Hello, message 1 from Rayon task {}", i),
            format!("This is message 2 from task {}", i),
            format!("This is message 3 from task {}", i),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            println!("Task {} sent a message", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Parallel processing completed");

    // Wait for the receiver thread to finish
    receiver_handle.join().unwrap();
}
