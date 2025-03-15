# Rust Collections: Vec, HashMap, and More

Rust's standard library includes several collection types that provide efficient ways to store and manipulate groups of values. Let's explore the most commonly used collections and their features.

## `Vec<T>`: A Growable Array

`Vec<T>` (vector) is one of the most frequently used collections in Rust. It's a growable, heap-allocated array.

### Creating a Vector

```rust
// Create an empty vector
let mut v: Vec<i32> = Vec::new();

// Create using the vec! macro
let v = vec![1, 2, 3, 4, 5];

// With initial capacity
let mut v = Vec::with_capacity(10);
```

### Adding Elements

```rust
let mut v = Vec::new();

// Add to the end
v.push(1);
v.push(2);
v.push(3);

// Add multiple elements
v.extend(vec![4, 5, 6]);

// Insert at specific position
v.insert(1, 10); // Insert 10 at index 1
```

### Accessing Elements

```rust
let v = vec![1, 2, 3, 4, 5];

// Using indexing (panics if out of bounds)
let third = v[2];

// Using get() (returns Option)
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// First and last elements
let first = v.first(); // Returns Option<&T>
let last = v.last();   // Returns Option<&T>
```

### Removing Elements

```rust
let mut v = vec![1, 2, 3, 4, 5];

// Remove and return the last element
let last = v.pop(); // Returns Option<T>

// Remove element at index
let second = v.remove(1); // Returns the removed element

// Clear all elements
v.clear();
```

### Iteration

```rust
let v = vec![1, 2, 3, 4, 5];

// Iterate over references
for item in &v {
    println!("{}", item);
}

// Iterate over mutable references
let mut v = vec![1, 2, 3, 4, 5];
for item in &mut v {
    *item *= 2;
}

// Consume the vector and iterate over owned values
for item in v {
    println!("{}", item);
}
```

## String and &str

While not strictly collections, strings deserve mention here as they're sequences of characters.

### String vs &str

- `String`: Owned, growable UTF-8 string
- `&str`: Borrowed string slice, view into a string

```rust
// Creating a String
let s = String::new();
let s = "hello".to_string();
let s = String::from("hello");

// Creating string slices
let s = "hello"; // &str literal
let slice = &s[1..3]; // "el"

// String operations
let mut s = String::from("Hello");
s.push_str(", world!"); // Append a string slice
s.push('!');  // Append a single character

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 is moved here
```

## HashMap<K, V>: Key-Value Storage

`HashMap<K, V>` provides an efficient key-value store.

### Creating a HashMap

```rust
use std::collections::HashMap;

// Empty HashMap
let mut scores = HashMap::new();

// Insert key-value pairs
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// From iterators of tuples
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### Accessing Values

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Using get (returns Option<&V>)
let blue_score = scores.get("Blue");

// Using the entry API
let score = scores.entry(String::from("Yellow")).or_insert(50);
*score += 10; // Now Yellow's score is 60
```

### Updating a HashMap

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Overwrite existing value
scores.insert(String::from("Blue"), 25);

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);

// Update value based on old value
let text = "hello world wonderful world";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}
// {"hello": 1, "world": 2, "wonderful": 1}
```

### Removing Elements

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Remove a key and its value
scores.remove("Blue");
```

## HashSet<T>: A Collection of Unique Values

`HashSet<T>` is implemented as a `HashMap<T, ()>` where only the keys are used.

```rust
use std::collections::HashSet;

// Create a HashSet
let mut langs = HashSet::new();
langs.insert("Rust");
langs.insert("C++");
langs.insert("Python");

// Check if contains a value
if langs.contains("Rust") {
    println!("Rust is in the set!");
}

// Remove a value
langs.remove("C++");

// Set operations
let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let set2: HashSet<_> = vec![3, 4, 5].into_iter().collect();

// Union
let union: HashSet<_> = set1.union(&set2).cloned().collect();
// {1, 2, 3, 4, 5}

// Intersection
let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
// {3}

// Difference
let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
// {1, 2}
```

## BTreeMap and BTreeSet

Similar to `HashMap` and `HashSet` but keep their keys sorted:

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "three");
map.insert(1, "one");
map.insert(2, "two");

// Iterate in sorted key order
for (key, value) in &map {
    println!("{}: {}", key, value);
}
// Output:
// 1: one
// 2: two
// 3: three
```

## `VecDeque<T>`: A Double-Ended Queue

`VecDeque<T>` provides efficient insertions and removals from both ends of the collection.

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();

// Add to back or front
deque.push_back(1);
deque.push_front(0);
deque.push_back(2);

// Remove from back or front
let first = deque.pop_front(); // Some(0)
let last = deque.pop_back();   // Some(2)
```

## Additional Collections

Rust's standard library also includes:

- `BinaryHeap<T>`: A priority queue where the maximum value is always at the front
- `LinkedList<T>`: A doubly-linked list
- `std::collections::btree_map::BTreeMap<K, V>`: A map sorted by keys

## Performance Considerations

Each collection type has different performance characteristics:

| Collection | Strengths                               | Use When You Need                     |
| ---------- | --------------------------------------- | ------------------------------------- |
| Vec        | Fast random access, efficient appending | Sequential data, stack behavior       |
| HashMap    | Fast average case lookup/insert/delete  | Key-value lookups, no ordering needed |
| HashSet    | Fast membership testing                 | Unique values, set operations         |
| BTreeMap   | Ordered keys, consistent performance    | Sorted data, range queries            |
| VecDeque   | Efficient operations at both ends       | Queue or deque behavior               |

## Memory Management in Collections

All standard library collections in Rust manage their memory automatically through the ownership system:

- When a collection is dropped, all elements it owns are also dropped
- Collections can store borrowed data (`&T`) but must respect lifetimes
- For collections that own their data, the `T` must implement `Drop` to clean up resources properly

```rust
// Collection of strings
let v = vec![String::from("hello"), String::from("world")];
// When v goes out of scope, all Strings are properly cleaned up

// Collection of references needs lifetime specifiers
fn process_names<'a>(names: &[&'a str]) -> Vec<&'a str> {
    names.iter().filter(|name| name.len() > 5).cloned().collect()
}
```

Would you like me to cover any specific aspect of collections in more detail?
