# Concurrency in Rust

Rust's approach to concurrency is summed up by the slogan "`fearless concurrency`" - **it leverages the type system and ownership rules to prevent data races and threading issues at compile time**. Let's break down the key components:

## Threads

Rust provides native threads through the standard library:

```rust
use std::thread;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        // This code runs in a separate thread
        println!("Hello from a thread!");
    });

    // Wait for the thread to finish
    handle.join().unwrap();
}
```

Key concepts with threads:

1. **Thread Safety**: The closure passed to `thread::spawn` must satisfy the `Send` bound, meaning it's safe to transfer across thread boundaries.

2. **Ownership and Threads**: When you spawn a thread with a closure that captures variables from its environment, the closure takes ownership of those variables:

```rust
let v = vec![1, 2, 3];

// This won't compile without 'move'
let handle = thread::spawn(move || {
    println!("Vector: {:?}", v);
});

// Can't use v here anymore
```

3. **Thread Builders**: For more control over thread creation:

```rust
let handle = thread::Builder::new()
    .name("custom-thread".to_string())
    .stack_size(4 * 1024 * 1024) // 4MB stack
    .spawn(|| {
        // thread code
    })
    .unwrap();
```

## Mutexes (Mutual Exclusion)

To share mutable data between threads, Rust provides mutexes:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a mutex-protected counter
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc to share ownership across threads
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to access the data
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // Mutex automatically unlocks when `num` goes out of scope
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Key concepts with mutexes:

1. **Arc**: Atomic Reference Counting (`Arc`) provides thread-safe shared ownership of a value. We need it to share the mutex between threads.

2. **Mutex API**:

   - `lock()` returns a `MutexGuard` that provides access to the data
   - The lock is automatically released when the guard is dropped (goes out of scope)
   - If another thread panics while holding the lock, `lock()` will return an error

3. **Deadlock Prevention**: Be careful about lock ordering to avoid deadlocks - Rust can't prevent these at compile time.

4. **Read-Write Locks**: `RwLock` provides more concurrency by allowing multiple readers or a single writer:

```rust
use std::sync::RwLock;

let data = RwLock::new(vec![1, 2, 3]);

// Multiple readers can access concurrently
let r1 = data.read().unwrap();
let r2 = data.read().unwrap();

// Only one writer, and no readers can exist simultaneously
let mut w = data.write().unwrap();
w.push(4);
```

## Channels

Channels provide a way to transfer data between threads:

```rust
use std::sync::mpsc; // multi-producer, single-consumer
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Clone sender for multiple producer threads
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("hello from thread 1").unwrap();
    });

    thread::spawn(move || {
        tx1.send("hello from thread 2").unwrap();
    });

    // Receive messages
    for _ in 0..2 {
        println!("Got: {}", rx.recv().unwrap());
    }
}
```

Key concepts with channels:

1. **Sender and Receiver**: The channel has two endpoints:

   - `Sender<T>` for sending values of type T
   - `Receiver<T>` for receiving them

2. **Channel Types**:
   - Standard channels (`mpsc::channel`) are unbounded
   - Synchronous channels (`mpsc::sync_channel(n)`) have a fixed capacity
3. **Error Handling**:

   - `send()` returns `Err` if the receiver has been dropped
   - `recv()` returns `Err` if all senders have been dropped

4. **Multiple Producers**: You can clone `Sender` to have multiple producer threads

5. **Iteration**: You can iterate over a channel to receive all messages until all senders are dropped:

```rust
for received in rx {
    println!("Got: {}", received);
}
```

## Advanced Patterns

1. **Atomic Types**: For simple shared counters or flags without mutex overhead:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

let counter = Arc::new(AtomicUsize::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    counter_clone.fetch_add(1, Ordering::SeqCst);
});

println!("Counter: {}", counter.load(Ordering::SeqCst));
```

2. **Barriers**: Synchronize multiple threads at a specific point:

```rust
use std::sync::{Arc, Barrier};

let barrier = Arc::new(Barrier::new(3));
// All three threads will wait at the barrier until all have reached it
```

3. **Condition Variables**: For waiting until a condition becomes true:

```rust
use std::sync::{Arc, Mutex, Condvar};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
// One thread can wait until another sets the flag to true
```

4. **Parking Lot Crate**: More efficient mutex implementations:

```rust
use parking_lot::Mutex;

let mutex = Mutex::new(0);
// No unwrap needed, panics on poisoning
*mutex.lock() += 1;
```

Rust's concurrency primitives, combined with its ownership system, make it possible to write concurrent code that's both safe and efficient. The compiler prevents common concurrency errors like data races, while still allowing you to build sophisticated multi-threaded systems.
