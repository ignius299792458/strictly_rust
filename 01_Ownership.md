# Ownership
---

## 1️⃣ What is Ownership?

**Ownership** in Rust means:

> Each value in Rust has **exactly one variable (owner)** that is responsible for it.
> When the owner goes out of scope, the value is automatically **dropped/freed**.

---

### 🔹 Key Rules of Ownership

1. **Each value has a single owner** at a time.
2. **When the owner goes out of scope**, Rust automatically cleans up the value (calls `drop`).
3. **Ownership can be transferred (moved)**, but then the old owner can no longer use the value.
4. You can **borrow** a value without taking ownership (immutable or mutable).

---

## 2️⃣ Example: Ownership Transfer (Move)

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1;                     // ownership moves to s2

    // println!("{}", s1);  // ❌ error: s1 no longer owns it
    println!("{}", s2);      // ✅ works
} // s2 goes out of scope, memory is freed automatically
```

* `s1` originally owns `"hello"`.
* `s2 = s1` **moves ownership** to `s2`.
* After the move, `s1` is invalid (cannot use it).
* When `s2` goes out of scope, Rust calls `drop` on `"hello"` automatically.

---

## 3️⃣ Borrowing vs Ownership

* **Ownership** = who is responsible for the value.
* **Borrowing** = temporary access without taking ownership.

```rust
let s = String::from("hello");
let r = &s; // borrow: r can read s, but doesn't own it
println!("{}", r);
```

* `r` cannot drop `s`.
* `s` remains the owner.

---

## 4️⃣ Ownership Analogy

* Imagine a **library book**:

| Concept   | Analogy                                                                                                  |
| --------- | -------------------------------------------------------------------------------------------------------- |
| Ownership | You have the library card for the book. You are responsible for returning it.                            |
| Move      | You give your library card to someone else. Now they are responsible; you can’t use it.                  |
| Borrow    | You lend the book to a friend temporarily. You still own it. They can read it, maybe write (if mutable). |

---

## 5️⃣ Why Ownership Matters

1. **Memory safety without garbage collector**

   * No double free, no dangling pointers.
2. **Deterministic cleanup**

   * When a variable goes out of scope → memory freed immediately.
3. **Thread safety**

   * Ownership rules prevent data races by design.
4. **Works with borrowing and references**

   * You can safely share or mutate data temporarily.

---

### 🔹 Quick Rust example of all three together

```rust
fn main() {
    let s = String::from("hello");  // s owns the string
    let r1 = &s;                     // immutable borrow
    println!("r1 reads: {}", r1);

    let r2 = &s;                     // another immutable borrow
    println!("r2 reads: {}", r2);

    // let r3 = &mut s;              // ❌ cannot borrow mutably while immutable borrows exist

} // s goes out of scope here → string memory is freed
```

---

✅ **Key takeaway**:

* **Ownership** = who owns the value and is responsible for freeing it.
* **Borrowing / References** = temporary use without taking ownership.
* Rust’s compiler enforces ownership strictly → prevents memory bugs at compile time.

---
