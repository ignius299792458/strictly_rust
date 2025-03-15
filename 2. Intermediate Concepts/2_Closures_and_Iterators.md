# Closures and Iterators in Rust: Functional Programming Patterns

Rust incorporates many functional programming concepts, with closures and iterators being two of the most powerful. Let's explore how these features work and how they enable functional programming patterns in Rust.

## Closures

`Closures` are **_anonymous functions that can capture their environment_**. They're extremely useful for **creating inline functions** and **working with higher-order functions**.

### Basic Syntax

```rust
// Basic closure that takes two parameters
let add = |x, y| x + y; // inline fn
let result = add(5, 3); // 8

// Closure with explicit type annotations
let multiply: |i32, i32| -> i32 = |x, y| x * y;
let result = multiply(5, 3); // 15

// Closure with no parameters
let say_hello = || println!("Hello!");
say_hello(); // Prints: Hello!
```

### Capturing the Environment

Closures can capture variables from their surrounding scope:

```rust
let x = 4;
let equal_to_x = |z| z == x;
let y = 4;
assert!(equal_to_x(y)); // true
```

Rust closures can capture variables in three ways:

1. **FnOnce**: Consumes the captured variables (**takes ownership**)
2. **FnMut**: Borrows values mutably
3. **Fn**: Borrows values immutably

```rust
// FnOnce example - consumes the captured variable
let s = String::from("hello");
let consume = move || {
    println!("Consumed: {}", s);
    // s is moved into the closure here
};
consume();
// s is no longer valid here

// FnMut example - mutably borrows
let mut counter = 0;
let mut increment = || {
    counter += 1;
    println!("Counter: {}", counter);
};
increment(); // Counter: 1
increment(); // Counter: 2

// Fn example - immutably borrows
let name = String::from("Rust");
let greet = || println!("Hello, {}!", name);
greet(); // Hello, Rust!
// name is still accessible here
```

### Closure Type Inference

Rust can infer the types and borrowing behavior of closures based on how they're used:

```rust
let list = vec![1, 2, 3];
let only_borrows = || println!("Length: {}", list.len());
only_borrows();

let mut mut_list = vec![1, 2, 3];
let mut borrows_mutably = || mut_list.push(4);
borrows_mutably();

// The `move` keyword forces ownership
let owned_list = vec![1, 2, 3];
let owns_data = move || println!("Owned list: {:?}", owned_list);
owns_data();
// owned_list is no longer valid here
```

## Iterators

Iterators allow you to process sequences of items one at a time. Rust's iterators are:

- Lazy (values are generated on-demand)
- Zero-cost (abstractions compile to optimal code)
- Composable (can be chained together)

### Creating an Iterator

```rust
let v = vec![1, 2, 3];
let iter = v.iter(); // Creates an iterator over references

// Iterate over values with for loop
for val in iter {
    println!("{}", val);
}
```

### Iterator Methods

The `Iterator` trait provides many useful methods:

```rust
let v = vec![1, 2, 3, 4, 5];

// map: Transform each element
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6, 8, 10]

// filter: Keep elements that match a predicate
let even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
// [2, 4]

// fold: Accumulate a value
let sum = v.iter().fold(0, |acc, x| acc + x);
// 15
```

### Common Iterator Methods

```rust
let numbers = vec![1, 2, 3, 4, 5];

// any: Check if any element satisfies a condition
let has_even = numbers.iter().any(|x| x % 2 == 0); // true

// all: Check if all elements satisfy a condition
let all_positive = numbers.iter().all(|x| *x > 0); // true

// find: Get the first element that matches a predicate
let first_even = numbers.iter().find(|x| *x % 2 == 0); // Some(&2)

// position: Find the index of the first matching element
let pos = numbers.iter().position(|x| *x == 3); // Some(2)

// count: Count the number of elements
let count = numbers.iter().count(); // 5

// sum, min, max: Common numeric operations
let sum: i32 = numbers.iter().sum(); // 15
let min = numbers.iter().min(); // Some(&1)
let max = numbers.iter().max(); // Some(&5)
```

### Chaining Iterator Methods

Iterator methods can be chained for complex operations:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let sum_of_squared_even: i32 = numbers.iter()
    .filter(|&x| x % 2 == 0)    // Keep even numbers
    .map(|&x| x * x)            // Square them
    .sum();                     // Sum them
// 220 (4 + 16 + 36 + 64 + 100)
```

### Creating Iterators

Different ways to create iterators:

```rust
// From collections
let vec_iter = vec![1, 2, 3].iter();
let hash_map_iter = hash_map.iter();

// Ranges
let range_iter = (1..10).into_iter(); // 1 through 9

// From functions
let lines = "Hello\nWorld".lines(); // Iterator over lines

// Infinite iterators
let repeating = std::iter::repeat(5); // 5, 5, 5, ...
let incremental = (0..).into_iter(); // 0, 1, 2, ...
```

### Consuming and Non-Consuming Adaptors

- **Consuming adaptors**: Call `next()` and consume the iterator (like `sum()`, `collect()`)
- **Iterator adaptors**: Produce new iterators (like `map()`, `filter()`)

```rust
let v = vec![1, 2, 3];

// collect() is a consuming adaptor
let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();

// Non-consuming adaptor chain only prepares the computation
let iter = v.iter().map(|x| x * 2).filter(|x| *x > 5);
// No computation happens until we consume the iterator
let result: Vec<_> = iter.collect(); // [6]
```

## Combining Closures and Iterators

Closures and iterators work well together for powerful functional-style operations:

```rust
let teams = vec![
    ("Bears", 13),
    ("Lions", 25),
    ("Tigers", 17),
    ("Eagles", 25),
];

// Find teams with the highest score
let max_score = teams.iter().map(|(_, score)| score).max().unwrap();

let winners: Vec<_> = teams.iter()
    .filter(|(_, score)| score == max_score)
    .map(|(name, _)| name)
    .collect();
// ["Lions", "Eagles"]
```

## Custom Iterators

You can implement the `Iterator` trait for your own types:

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage
let counter = Counter::new(5);
for num in counter {
    println!("{}", num); // Prints 1, 2, 3, 4, 5
}
```

## Performance

Despite their high-level abstractions, Rust's iterators and closures are compiled to highly optimized code, often outperforming equivalent manual loops due to optimizations like loop unrolling and bounds-check elimination.

Let me know if you'd like more specific examples of using iterators and closures for particular tasks or more advanced functional programming patterns in Rust!
