# `Smart Pointers` in Rust: Box, Rc, and RefCell

`Smart pointers` in Rust are data structures that **act like pointers but come with additional capabilities and guarantees**.

Unlike `raw pointers` in languages like C/C++, Rust's `smart pointers` enforce memory safety and prevent common issues like **_dangling pointers, double frees, and memory leaks_**.

## `Smart Pointers` vs. Normal (Raw) Pointers

Before diving into specific `smart pointers`, let's understand how they differ from normal pointers:

### `Raw Pointers` in C/C++:

- Are just memory addresses
- Provide no safety guarantees
- Allow null pointers, dangling references
- Require manual memory management
- Can be arbitrarily manipulated with pointer arithmetic

### Rust's `Smart Pointers`:

- Implement the `Deref` and `Drop` traits
- Enforce memory safety at compile time
- Manage their memory automatically when they go out of scope
- Provide additional semantics (ownership, borrowing, interior mutability)
- Are zero-cost abstractions in most cases

Now, let's explore the **`3 most common smart pointers`** in Rust:

## Box<T>: Heap Allocation

`Box<T>` is the simplest `smart pointer` in Rust. It **allocates a value** on the **heap instead of the stack**.

### Use Cases for Box<T>:

1. **When you have a large value** and want to avoid copying it when passing it around:

   ```rust
   let large_data = Box::new([0u8; 1_000_000]);
   ```

2. **When you need a known-size type for recursive data structures**:

   ```rust
   enum List {
       Cons(i32, Box<List>),
       Nil,
   }
   ```

3. **When you want to store a trait object** with dynamic dispatch:
   ```rust
   let animal: Box<dyn Animal> = Box::new(Dog::new());
   ```

### Box<T> Example:

```rust
fn main() {
    // Stack allocated integer
    let x = 5;
    println!("x on stack: {}", x);

    // Heap allocated integer
    let y = Box::new(5);
    println!("y on heap: {}", *y);  // Dereference with *

    // Box is dropped here, memory is automatically freed
}
```

## Rc<T>: Reference Counting

`Rc<T>` (`Reference Counted`) allows a value to have multiple owners. It keeps track of how many references to a value exist and cleans up the value when no references remain.

### Use Cases for Rc<T>:

1. **When data needs to be shared** but not modified:

   ```rust
   let shared_data = Rc::new(vec![1, 2, 3]);
   ```

2. **In data structures where a node might have multiple parents** (like graphs):
   ```rust
   struct Node {
       value: i32,
       children: Vec<Rc<Node>>,
       parent: Option<Rc<Node>>,
   }
   ```

### Rc<T> Example:

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(String::from("Hello"));
    println!("References: {}", Rc::strong_count(&original)); // 1

    {
        let cloned = Rc::clone(&original);
        println!("References: {}", Rc::strong_count(&original)); // 2
        println!("Shared value: {}", *cloned);
    } // cloned goes out of scope here

    println!("References: {}", Rc::strong_count(&original)); // 1
    // When count reaches 0, the String is dropped
}
```

**Key limitations**:

- `Rc<T>` only works in a single thread
- `Rc<T>` only allows immutable references - you can't modify the contents

## RefCell<T>: Interior Mutability

`RefCell<T>` allows mutable borrows checked at runtime. It enables you to mutate data even when there are immutable references to that data.

### Use Cases for RefCell<T>:

1. **When you need to mutate data behind a shared reference**:

   ```rust
   let data = RefCell::new(vec![1, 2, 3]);
   let reference = &data;
   reference.borrow_mut().push(4); // Still works!
   ```

2. **When implementing mock objects for testing**:
   ```rust
   struct MockObject {
       call_count: RefCell<u32>,
   }
   ```

### RefCell<T> Example:

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // Multiple immutable borrows are fine
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("r1: {}, r2: {}", r1, r2);

    // These borrows are dropped at the end of their scope
    drop(r1);
    drop(r2);

    // Now we can mutably borrow
    let mut m = data.borrow_mut();
    *m += 1;
    println!("m: {}", m);

    // This would panic - can't have both mutable and immutable borrows:
    // let r3 = data.borrow(); // RUNTIME PANIC!
}
```

Key points about `RefCell<T>`:

- Enforces borrowing rules at runtime (can panic!)
- Works only in a single thread
- Adds a small runtime cost

## Combining `Smart Pointers`

`Smart pointers` can be combined to overcome their individual limitations:

### Rc<RefCell<T>>: Shared Mutable Data

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Shared ownership with mutable contents
    let shared_mutable = Rc::new(RefCell::new(vec![1, 2, 3]));

    let clone1 = Rc::clone(&shared_mutable);
    let clone2 = Rc::clone(&shared_mutable);

    // All clones can mutate the shared value
    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);

    println!("Shared data: {:?}", shared_mutable.borrow());
    // Output: Shared data: [1, 2, 3, 4, 5]
}
```

## Weak<T>: Preventing Reference Cycles

`Weak<T>` is a version of `Rc<T>` that doesn't prevent a value from being dropped. It's used to break reference cycles that would otherwise cause memory leaks.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: Option<Weak<RefCell<Node>>>,
    children: RefCell<Vec<Rc<RefCell<Node>>>>,
}

fn main() {
    let leaf = Rc::new(RefCell::new(Node {
        value: 3,
        parent: None,
        children: RefCell::new(vec![]),
    }));

    let branch = Rc::new(RefCell::new(Node {
        value: 5,
        parent: None,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    }));

    // Set leaf's parent to branch without creating a cycle
    leaf.borrow_mut().parent = Some(Rc::downgrade(&branch));
}
```

## How to Choose the Right Smart Pointer

- **Box<T>** - When you need heap allocation with single ownership
- **Rc<T>** - When you need shared ownership of immutable data
- **RefCell<T>** - When you need interior mutability
- **Rc<RefCell<T>>** - When you need shared ownership of mutable data
- **Weak<T>** - When you need to break reference cycles

## Performance Implications

Unlike `raw pointers`, `smart pointers` in Rust do come with some costs:

- **Box<T>**: Minimal overhead, almost the same as `raw pointers`
- **Rc<T>**: Adds reference counting overhead
- **RefCell<T>**: Adds runtime borrow checking overhead
- **Rc<RefCell<T>>**: Combines both overheads

However, these costs are often negligible compared to the memory safety benefits they provide.
