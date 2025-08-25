# `char`

## 1. **What is a `char` in Rust?**

* In Rust, a `char` is a **Unicode Scalar Value**, not a single byte.
* It takes **4 bytes (32 bits)** of memory.
* Represents **any valid Unicode character**, including emojis, accented characters, CJK, etc.

👉 Example:

```rust
fn main() {
    let a: char = 'A';       // Latin letter
    let b: char = 'π';       // Greek letter
    let c: char = '🚀';      // Emoji
    let d: char = 'न';       // Devanagari
    println!("{}, {}, {}, {}", a, b, c, d);
}
```

✅ Output:

```
A, π, 🚀, न
```

---

## 2. **Memory Size**

```rust
fn main() {
    use std::mem::size_of;
    println!("{}", size_of::<char>()); // 4
}
```

* Always **4 bytes**, unlike C/C++ where `char` is often 1 byte.

---

## 3. **Range of Values**

* Rust `char` is a **Unicode scalar value**:

  * From `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`
  * Excludes surrogate pairs (`U+D800` to `U+DFFF`)

Example:

```rust
fn main() {
    let max = char::from_u32(0x10FFFF).unwrap();
    println!("{}", max); // '\u{10FFFF}'
}
```

---

## 4. **Char Literals**

```rust
fn main() {
    let c1 = 'a';             // normal
    let c2 = '\n';            // escape sequence
    let c3 = '\'';            // single quote
    let c4 = '\\';            // backslash
    let c5 = '\u{03C0}';      // Unicode: π
    let c6 = '\x41';          // ASCII hex: A
    println!("{}, {}, {}, {}, {}, {}", c1, c2, c3, c4, c5, c6);
}
```

---

## 5. **Converting Between Types**

### Char ↔ Integer

```rust
fn main() {
    let c = 'A';
    let code: u32 = c as u32;    // 65
    let back: char = char::from_u32(65).unwrap();
    println!("{}, {}", code, back); // 65, A
}
```

### Char ↔ String

```rust
fn main() {
    let c = '🦀';
    let s: String = c.to_string();    // char → String
    let s2: &str = c.encode_utf8(&mut [0u8; 4]); // char → str (UTF-8)
    println!("{}, {}", s, s2);
}
```

---

## 6. **Iterating Over Characters**

```rust
fn main() {
    let s = "नमस्ते"; // UTF-8 string
    for c in s.chars() {
        println!("{}", c);
    }
}
```

✅ Output:

```
न
म
स
्
त
े
```

⚠️ Notice: Some grapheme clusters (like emoji + modifier) can be more than 1 `char`.

---

## 7. **Comparison and Ordering**

```rust
fn main() {
    println!("{}", 'a' < 'b');   // true
    println!("{}", 'ß' < 'z');   // true, because Unicode scalar ordering
}
```

---

## 8. **Methods on `char`**

Rust’s `char` has **tons of methods** in `std::char`:

### Classification

```rust
fn main() {
    let c = 'A';
    println!("{}", c.is_alphabetic()); // true
    println!("{}", c.is_numeric());    // false
    println!("{}", c.is_alphanumeric());// true
    println!("{}", c.is_uppercase());   // true
    println!("{}", c.is_lowercase());   // false
    println!("{}", c.is_whitespace());  // false
}
```

### Conversions

```rust
fn main() {
    println!("{}", 'A'.to_lowercase().to_string()); // a
    println!("{}", 'a'.to_uppercase().to_string()); // A
}
```

---

## 9. **Char in Pattern Matching**

```rust
fn main() {
    let c = 'x';
    match c {
        'a'..='z' => println!("Lowercase letter"),
        'A'..='Z' => println!("Uppercase letter"),
        '0'..='9' => println!("Digit"),
        _ => println!("Other"),
    }
}
```

---

## 10. **Char and UTF-8 Encoding**

```rust
fn main() {
    let mut buf = [0u8; 4];
    let c = '💖';
    let encoded = c.encode_utf8(&mut buf);
    println!("Char: {}, UTF-8 bytes: {:?}", c, encoded.as_bytes());
}
```

✅ Example Output:

```
Char: 💖, UTF-8 bytes: [240, 159, 146, 150]
```

---

## 11. **Char and Graphemes**

⚠️ A single **grapheme cluster** may be multiple `char`s.
Example:

```rust
fn main() {
    let s = "🇳🇵"; // Nepali flag
    println!("chars: {}", s.chars().count()); // 2 (regional indicators)
    println!("len: {}", s.len());             // 8 bytes
}
```

For grapheme clusters (what humans see as a character), use [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation):

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "🇳🇵";
    let graphemes = s.graphemes(true).collect::<Vec<&str>>();
    println!("{:?}", graphemes); // ["🇳🇵"]
}
```

---

## 12. **Performance Considerations**

* `char` is **always 4 bytes**, so large collections (like `Vec<char>`) are memory heavy.
* Prefer **`&str` or `String`** (UTF-8 compact).
* Use `chars()` only when you truly need Unicode scalar iteration.

---

## 13. **When to Use `char`**

* Parsing a single symbol (like operators, tokens, commands).
* Character classification (digit, alphabetic, etc.).
* Converting between `char` and `u32` (Unicode handling).
* Rarely for storing large text — better to use `String`/`&str`.

---

✅ **Summary Table**

| Aspect        | Details                                                   |
| ------------- | --------------------------------------------------------- |
| Size          | 4 bytes                                                   |
| Type          | Unicode scalar value (not byte)                           |
| Range         | U+0000 → U+10FFFF (excluding surrogates)                  |
| Literal forms | `'a'`, `'\n'`, `'\u{03C0}'`, `'\x41'`                     |
| Conversions   | `as u32`, `from_u32`, `.to_string()`, `.encode_utf8()`    |
| Iteration     | `s.chars()`, but graphemes require `unicode-segmentation` |
| Use cases     | Parsing, tokenizing, classification, Unicode logic        |

---
