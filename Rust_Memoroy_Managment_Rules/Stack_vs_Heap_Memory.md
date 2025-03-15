# Stack vs. Heap Memory in Rust

In Rust, values can only be stored in two places in memory: the stack and the heap. Each has different characteristics that affect performance, memory management, and how you work with data.

## Stack Memory

The stack is a region of memory that operates in a Last-In-First-Out (LIFO) manner:

- **Fast allocation**: Memory allocation is just a pointer increment
- **Automatic cleanup**: Memory is automatically freed when a variable goes out of scope
- **Size limitations**: Values must have a known, fixed size at compile time
- **Scope-bound**: Stack memory is tied to the function or block where it's declared
- **Thread-specific**: Each thread has its own stack

In Rust, primitive types (integers, floats, booleans, chars), fixed-size arrays, tuples with fixed-size elements, and structs containing only stack-storable types are stored on the stack.

```rust
fn example() {
    let x = 42; // Integer stored on stack
    let y = [1, 2, 3, 4, 5]; // Fixed-size array on stack
    let z = (10, "hello"); // Tuple on stack (pointer to string data is on stack)
} // All stack memory automatically freed here
```

## Heap Memory

The heap is for data whose size is unknown at compile time or may change:

- **Dynamic allocation**: Memory size can be determined at runtime
- **Manual management**: In most languages, requires explicit allocation/deallocation (Rust handles this through ownership)
- **Slower access**: Slightly slower than stack access
- **Flexible lifetime**: Can live beyond the scope where it was created
- **Shared resource**: Accessible by multiple parts of the program

In Rust, types like `String`, `Vec`, `Box`, `Rc`, and any other types that can grow or shrink are stored on the heap.

```rust
fn example() {
    let s = String::from("hello"); // String data on heap, pointer on stack
    let v = vec![1, 2, 3]; // Vector data on heap, metadata on stack
    let b = Box::new(42); // Boxed integer on heap, pointer on stack
} // Rust automatically frees heap memory when the variables go out of scope
```

## What Makes Rust Special

Rust's ownership system combines these memory models:

1. **Stack values** follow simple copy or move semantics based on whether they implement the `Copy` trait
2. **Heap values** follow strict ownership rules:
   - Each value has exactly one owner
   - When the owner goes out of scope, the value is dropped (freed)
   - Ownership can be transferred (moved) or temporarily borrowed

This system gives Rust memory safety without garbage collection and precise control over memory layout and lifetimes.

## Stack and Heap Interaction

In practice, complex types usually have parts in both locations:

- A `String` has its length, capacity, and pointer metadata on the stack, but the actual character data on the heap
- A `Vec<T>` has its metadata on the stack but the elements it contains on the heap
- References (`&T`) are stack-based pointers to data that may be on either the stack or heap

Rust's borrow checker ensures these relationships maintain memory safety while giving you fine-grained control over where and how your data is stored.
