# Address and Reference

## In Rust

```rust
let x = 10;
let r2 = &x;
```

* `x` is a value (`10`) stored somewhere in memory.
* `&x` is **not the address of x** in the C sense — it is a **reference** to `x`.

So:

* In **C**: `&x` → literally the **raw address** of `x`.
* In **Rust**: `&x` → a **reference (safe handle)** to `x`.

  * Internally, yes, it contains the memory address.
  * But the compiler enforces lifetimes & borrowing rules, so you can’t misuse it.

---

## Borrowing section meaning

When you write:

```rust
let r2 = &x;
```

* You are **borrowing `x` immutably**.
* `r2` is a reference (an alias to `x`).
* You can read `x` through `r2` but not modify it.

If you try:

```rust
*r2 = 20; // ❌ compile error: cannot assign to borrowed `x`
```

Rust rejects it, because `&x` is an **immutable borrow**.

---

## Analogy

* **C `&x`** → “Here’s the raw street address of the house. You can blow it up if you want.”
* **Rust `&x`** → “Here’s a library card that lets you look inside the house safely, but you can’t wreck it.”

So → in Rust, `&x` is **a reference (borrow of value x)**, not “reference of address of x”.
It’s directly “reference to the value of `x`”.

---

## ✅ TL;DR

* `&x` in Rust = borrow `x` safely (immutable reference).
* `&x` in C = raw memory address of `x`.
* Internally both hold the **address of `x`**, but Rust wraps it with **safety + rules**.

---

`x → address → reference (&x) → dereference (*r)`
