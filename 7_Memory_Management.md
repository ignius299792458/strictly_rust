## Memory Management in rust

Scopes are used frequently in Rust for:

1. Limiting the lifetime of variables
2. Creating temporary variables
3. Controlling when resources are freed
4. Demonstrating and enforcing lifetime relationships

**Summay**: This flow of execution and cleanup is part of what makes Rust memory-safe without a garbage collector. You know exactly when variables will be created and destroyed, following lexical scope rules.
