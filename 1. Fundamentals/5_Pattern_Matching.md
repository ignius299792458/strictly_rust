# Pattern Matching with Match Expressions in Rust

Match expressions are one of Rust's most powerful features. They allow you to compare a value against a series of patterns and execute code based on which pattern matches.

### Basic Syntax

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

The match expression compares a value against each pattern in turn and executes the code associated with the first matching pattern.
