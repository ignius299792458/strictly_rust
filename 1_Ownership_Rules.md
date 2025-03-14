## Ownership Rules

1. **_All values in Rust have owners_**. This fundamental rule applies to both primitive types like integers and complex types like String.
2. **_The difference is in what happens during assignment or function calls:_**

   1. **For non-Copy types (like String)**: ownership moves, and the original variable becomes invalid

   2. **For Copy types (like integers)**: the value is copied, and both the original and new variable remain valid with their own independent copies of the data

3. **_Primitives still follow ownership rules_**, but the "move" behavior is replaced with "copy" behavior:

```rust

// Example of ownership rules

let a = 5;      // 'a' owns the value 5
let b = a;      // The value is copied, so 'b' gets its own 5
                // 'a' still owns its original 5

fn foo(x: i32) { // x gets a copy of the value
    println!("{}", x);
} // x goes out of scope here, but it's just dropping a copy

foo(a);         // A copy of 'a' is passed to foo
println!("{}", a); // Still valid, prints 5

/*
CONCLUSION:

PRIMITIVES:
-> Rust enforces ownership for all types, but primitives are automatically copied instead of moved because they're small, stack-allocated values with no heap resources to manage, preserving memory safety while allowing more convenient usage.

COMPOSITIVE:
-> Composite types like String transfer exclusive ownership when assigned or passed to functions, preventing multiple references from managing the same heap memory and automatically freeing resources when the owner goes out of scope.
*/
```
