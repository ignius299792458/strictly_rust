# 📚 `String`

---

## 1. **What is a `String` in Rust?**

* `String` is an **owned, growable, UTF-8 encoded text type**.
* It lives on the **heap** (unlike `&str`, which is borrowed).
* You can mutate it, append, push, clear, etc.

👉 Example:

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}
```

✅ Output:

```
Hello, world!
```

---

## 2. **String vs \&str**

* `&str` = borrowed string slice (reference to UTF-8 data).
* `String` = owned, growable buffer on the heap.

```rust
fn main() {
    let slice: &str = "Hello";        // string literal (&'static str)
    let mut owned: String = slice.to_string(); // owned String
    owned.push('!'); 
    println!("{}, {}", slice, owned);
}
```

---

## 3. **Memory Representation**

A `String` is essentially:

```rust
struct String {
    vec: Vec<u8>, // underlying buffer of bytes
}
```

Internally it stores:

* **Pointer** to UTF-8 bytes
* **Length** (number of bytes used)
* **Capacity** (allocated space)

✅ Example:

```rust
fn main() {
    let s = String::from("Rust");
    println!("len={}, capacity={}", s.len(), s.capacity());
}
```

---

## 4. **Creating Strings**

Multiple ways:

```rust
fn main() {
    let s1 = String::new();                  // empty string
    let s2 = String::from("Hello");          // from literal
    let s3 = "Hi".to_string();               // from &str
    let s4: String = "Hola".into();          // into
    let s5 = format!("{} {}", "Hello", "Rust"); // format!
}
```

---

## 5. **Basic Operations**

```rust
fn main() {
    let mut s = String::from("Hello");

    s.push('!');             // add a char
    s.push_str(" Rust");     // add a &str
    s.insert(0, '🌟');        // insert at position
    s.pop();                 // remove last char
    s.truncate(6);           // shorten length
    s.clear();               // empty the string
}
```

---

## 6. **Indexing (⚠️ tricky!)**

Rust **does not allow `s[i]` indexing**, because strings are UTF-8 (chars have variable size).

✅ Instead:

```rust
fn main() {
    let s = String::from("नमस्ते");

    // Get nth char
    let c = s.chars().nth(2).unwrap();
    println!("{}", c); // स
}
```

---

## 7. **String Slicing**

```rust
fn main() {
    let s = String::from("Здравствуйте");
    let hello = &s[0..4]; // takes 2 chars ("Зд")
    println!("{}", hello);
}
```

⚠️ Slicing must align with UTF-8 character boundaries. Otherwise → panic.

---

## 8. **Concatenation**

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s3 = s1 + " " + &s2;   // s1 is moved here
    let s4 = format!("{} {}", s3, "Rust");
    println!("{}", s4);
}
```

---

## 9. **Iteration**

```rust
fn main() {
    let s = "नमस्ते";

    for b in s.bytes() {
        println!("byte: {}", b);
    }

    for c in s.chars() {
        println!("char: {}", c);
    }
}
```

👉 Grapheme clusters (like emojis + modifiers) need `unicode-segmentation` crate:

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    for g in "👨‍👩‍👧‍👦".graphemes(true) {
        println!("{}", g);
    }
}
```

---

## 10. **Conversions**

### String ↔ \&str

```rust
fn main() {
    let s: String = String::from("hello");
    let slice: &str = &s;      // borrow
    let s2: String = slice.to_string(); 
}
```

### String ↔ Vec<u8>

```rust
fn main() {
    let s = String::from("Rust");
    let bytes: Vec<u8> = s.into_bytes();
    let s2 = String::from_utf8(bytes).unwrap();
}
```

### String ↔ OsString / PathBuf (for filesystem)

```rust
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    let s = String::from("file.txt");
    let os: OsString = s.clone().into();
    let path: PathBuf = s.into();
    println!("{:?}, {:?}", os, path);
}
```

---

## 11. **Useful Methods**

```rust
fn main() {
    let s = String::from(" Rust  ");

    println!("{}", s.len());          // 7 (bytes)
    println!("{}", s.is_empty());     // false
    println!("{}", s.trim());         // "Rust"
    println!("{}", s.starts_with(" ")); // true
    println!("{}", s.ends_with(" "));   // true
    println!("{}", s.replace("Rust", "🦀"));
}
```

---

## 12. **Ownership and Borrowing**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;         // move (s1 invalid now)
    let s3 = s2.clone(); // deep copy
    println!("{}", s3);
}
```

---

## 13. **Capacity and Performance**

```rust
fn main() {
    let mut s = String::with_capacity(10);
    s.push_str("hello");
    println!("len={}, cap={}", s.len(), s.capacity());
}
```

* Use `with_capacity` if you know size ahead of time.
* Prevents multiple reallocations.

---

## 14. **Pattern Matching**

```rust
fn main() {
    let s = String::from("rustacean");
    match s.as_str() {
        "rustacean" => println!("Crab!"),
        _ => println!("Not a crab"),
    }
}
```

---

## 15. **Strings in Enums and Structs**

```rust
struct User {
    name: String,
    email: String,
}
```

⚠️ If immutable and known at compile time, prefer `&'static str`.

---

## 16. **Strings in FFI / System Boundaries**

* Convert `String` ↔ `CString` for C APIs.

```rust
use std::ffi::CString;

fn main() {
    let s = CString::new("hello").unwrap();
    println!("{:?}", s);
}
```

---

## 17. **Advanced: Rope / Cow / Rc<str>**

* `String` isn’t always best for massive text.
* Alternatives:

  * [`ropey`](https://crates.io/crates/ropey) → for huge text editing.
  * `Cow<'a, str>` → clone-on-write optimization.
  * `Rc<str>` / `Arc<str>` → reference-counted immutable strings.

---

## 18. **Common Pitfalls**

1. **Indexing error** → Rust prevents direct `s[i]` indexing.
2. **UTF-8 length mismatch** → `len()` returns bytes, not chars.
3. **Performance** → avoid unnecessary clones, prefer `&str` if not mutating.
4. **Capacity growth** → exponential doubling when capacity exceeded.

---

## 19. **When to Use `String` vs `&str`**

| Use Case                      | Type           |
| ----------------------------- | -------------- |
| Known at compile time, static | `&'static str` |
| Borrowing immutable text      | `&str`         |
| Need ownership & mutation     | `String`       |
| Passing to C API              | `CString`      |

---

## 20. **Summary**

* ✅ `String` = growable, heap-allocated, UTF-8 text.
* ✅ Use `&str` for borrowed slices.
* ✅ Not indexable by number → must slice or iterate.
* ✅ Provides rich methods for manipulation.
* ✅ Watch out for performance & UTF-8 issues.

---
