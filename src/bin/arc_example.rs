use std::sync::{Arc, Mutex};
use std::thread;

struct SharedString {
    message: String,
    inner_counter: i32,
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.message, self.inner_counter)
    }
}

// Function that will be used as a function pointer
fn thread_function(shared_string: Arc<Mutex<SharedString>>, thread_id: i32) {
    let mut shared_string = shared_string.lock().unwrap();
    shared_string.inner_counter += 1;
    let message = format!("Thread function {} says hello, functionally!", thread_id);
    println!("{} (SharedString: {})", message, *shared_string);
}

fn main() {
    // in this case, the counter is a pointer to an integer, and the Mutex is on the
    // pointer to the counter, not the integer itself.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            let message = format!("Thread {} says hello!", i);
            println!("{} (Counter: {})", message, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
    main2();
}

fn main2() {
    // in this case, the shared_message is a pointer to a SharedString..., but the
    // Mutex is on the pointer to the shared_message, not the SharedString itself.
    let shared_message = Arc::new(Mutex::new(SharedString {
        message: "shared_string".to_string(),
        inner_counter: 0,
    }));
    let mut handles = vec![];

    for i in 0..5 {
        let shared_string = Arc::clone(&shared_message);
        // Using the function pointer instead of a closure
        let handle = thread::spawn(move || thread_function(shared_string, i));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Final SharedString value: {}",
        *shared_message.lock().unwrap()
    );
}
