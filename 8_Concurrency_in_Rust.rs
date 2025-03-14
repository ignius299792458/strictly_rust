// concurrency in rust

/*
Imports for the concurrency tools:
    Arc: Atomic Reference Counted pointer (thread-safe shared ownership)
    Mutex: Mutual exclusion lock (prevents data races)
    thread: Rust's threading library
 */
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /*
    Mutex::new(0): Creates a mutex that protects the value 0 from being raced.
    Arc::new(...): Wraps the mutex in an Arc, which allows multiple_threads to safely share ownership

    SUMMARY: This creates a thread-safe integer that can be shared and modified by multiple_threads
    */
    let counter = Arc::new(Mutex::new(0)); // Thread-safe reference counting with atomic operations

    // Creates an empty vector to store thread handles, which we'll use later to wait for threads to finish
    let mut handles = vec![];

    for _ in 0..10 {
        /*
        Makes a new reference to our shared counter
            Arc::clone() increases the reference count but points to the same data
            This is how we share data safely between threads
        */
        let counter = Arc::clone(&counter);

        /*
        thread::spawn(): Creates a new thread
        move ||: Creates a closure that takes ownership of variables it uses (in this case, counter)

        The move keyword is crucial - it transfers ownership of the counter clone into the thread
        */
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            *num *= 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
