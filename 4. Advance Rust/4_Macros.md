# Macros in Rust: Declarative and Procedural

Macros in Rust are powerful tools for metaprogramming, allowing you to write code that writes code. Rust has two main types of macros: declarative macros (also known as "macro_rules!") and procedural macros. Let's explore both types.

## Declarative Macros (macro_rules!)

Declarative macros allow you to write something similar to a match statement that operates on Rust syntax. They compare the input code against specific patterns and generate new code based on matches.

### Basic Syntax

```rust
macro_rules! my_macro {
    // Pattern => Expansion
    (pattern1) => {
        // Code to generate when pattern1 matches
    };
    (pattern2 $e:expr) => {
        // Code to generate when pattern2 matches with an expression
    };
    // More patterns...
}
```

### Common Designators for Pattern Matching

- `$x:expr`: Matches an expression
- `$x:ident`: Matches an identifier
- `$x:ty`: Matches a type
- `$x:pat`: Matches a pattern
- `$x:stmt`: Matches a statement
- `$x:block`: Matches a block of code
- `$x:literal`: Matches a literal (like numbers or strings)
- `$x:meta`: Matches a meta item (like attributes)

### Example: Simple Debug Macro

```rust
macro_rules! debug_print {
    ($value:expr) => {
        println!("{} = {:?}", stringify!($value), $value);
    };
}

fn main() {
    let x = 5;
    debug_print!(x); // Prints: x = 5
    debug_print!(2 + 3); // Prints: 2 + 3 = 5
}
```

### Example: Variable Arguments

```rust
macro_rules! create_function {
    // Simple pattern for creating a named function
    ($func_name:ident) => {
        fn $func_name() {
            println!("Called function: {}", stringify!($func_name));
        }
    };

    // Pattern with type and expression
    ($func_name:ident, $arg_type:ty, $body:expr) => {
        fn $func_name(arg: $arg_type) -> $arg_type {
            println!("Called function: {}", stringify!($func_name));
            $body
        }
    };
}

fn main() {
    // Create a simple function
    create_function!(hello);
    hello(); // Prints: Called function: hello

    // Create a function with an argument and body
    create_function!(double, i32, { arg * 2 });
    let result = double(5);
    println!("Result: {}", result); // Prints: Result: 10
}
```

### Repetition with Macros

You can match repeated patterns using `$(...)*` for zero or more repetitions, or `$(...)+` for one or more:

```rust
macro_rules! vector {
    // Empty case
    () => {
        Vec::new()
    };

    // Single element
    ($element:expr) => {
        {
            let mut v = Vec::new();
            v.push($element);
            v
        }
    };

    // Multiple elements with comma separation
    ($($element:expr),+) => {
        {
            let mut v = Vec::new();
            $(
                v.push($element);
            )+
            v
        }
    };
}

fn main() {
    let v1 = vector![];           // Empty vector
    let v2 = vector![1];          // Vector with one element
    let v3 = vector![1, 2, 3, 4]; // Vector with multiple elements

    println!("{:?}", v3); // Prints: [1, 2, 3, 4]
}
```

### Recursive Macros

Macros can call themselves recursively, which is useful for more complex patterns:

```rust
macro_rules! calculate {
    // Base case
    ($number:expr) => {
        $number
    };

    // Recursive case for addition
    ($number:expr, add $($rest:tt)*) => {
        $number + calculate!($($rest)*)
    };

    // Recursive case for multiplication
    ($number:expr, mul $($rest:tt)*) => {
        $number * calculate!($($rest)*)
    };
}

fn main() {
    let result = calculate!(5, add 3, mul 2, add 1);
    // This expands to: 5 + (3 * (2 + 1))
    println!("Result: {}", result); // Prints: Result: 14
}
```

## Procedural Macros

Procedural macros are more powerful than declarative macros. They operate on Rust's abstract syntax tree (AST) directly and can perform more complex transformations. They're defined in their own crates with the `proc-macro` crate type.

There are three types of procedural macros:

1. **Function-like macros**: Similar in usage to declarative macros
2. **Derive macros**: Add functionality to structs and enums with `#[derive(MacroName)]`
3. **Attribute macros**: Add new attributes with custom behavior

### Setting Up for Procedural Macros

To create a procedural macro, you need to set up a dedicated crate:

```toml
# Cargo.toml for a proc-macro crate
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
proc-macro2 = "1.0"
```

### 1. Function-like Procedural Macros

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let sql = parse_macro_input!(input as LitStr).value();

    // Simple SQL validation (in a real example, you'd do more)
    if !sql.to_lowercase().starts_with("select") {
        panic!("Only SELECT statements are supported");
    }

    // Generate code
    let output = quote! {
        {
            println!("Executing SQL: {}", #sql);
            // Here you would actually execute the SQL
            "Result of SQL query"
        }
    };

    output.into()
}
```

Usage:

```rust
let result = sql!("SELECT * FROM users");
```

### 2. Derive Macros

Derive macros automatically implement traits for structs and enums:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimpleDebug)]
pub fn derive_simple_debug(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Generate implementation
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {{ ... }}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}
```

Usage:

```rust
#[derive(SimpleDebug)]
struct Person {
    name: String,
    age: u32,
}
```

### 3. Attribute Macros

Attribute macros define new attributes:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse as a function
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;

    // Generate a new function with timing
    let expanded = quote! {
        fn #fn_name(#fn_inputs) #fn_output {
            let start = std::time::Instant::now();

            // Execute the original function body
            let result = {
                #fn_body
            };

            let duration = start.elapsed();
            println!("Function {} took {:?}", stringify!(#fn_name), duration);

            result
        }
    };

    TokenStream::from(expanded)
}
```

Usage:

```rust
#[timed]
fn expensive_function() -> u32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}
```

## Advanced Topics in Macros

### Hygiene

Macros in Rust are hygienic, which means that identifiers defined within a macro do not conflict with identifiers defined in the surrounding code:

```rust
macro_rules! create_variables {
    () => {
        let x = 5; // This 'x' is different from any 'x' outside
    }
}

fn main() {
    let x = 10;
    create_variables!();
    println!("x: {}", x); // Prints: x: 10
}
```

### Debugging Macros

You can use the `cargo expand` command (requires `cargo-expand` to be installed) to see the expanded code:

```
cargo install cargo-expand
cargo expand
```

### Best Practices for Macros

1. **Use macros sparingly**: Only use them when functions or traits won't work
2. **Document behavior carefully**: Make sure users understand how to use the macro
3. **Use consistent naming**: End function-like macros with `!`
4. **Provide good error messages**: Use panic messages that explain what went wrong
5. **Consider alternatives**: Can the same be achieved with a function, trait, or generic?

## Real-World Examples

### The `vec!` Macro

One of the most commonly used macros in Rust is `vec!`:

```rust
let v = vec![1, 2, 3];
```

This expands to something like:

```rust
let v = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
```

### Serde's Derive Macros

The popular Serde library uses procedural macros to automatically implement serialization and deserialization:

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}
```

Macros in Rust provide a powerful way to extend the language and reduce boilerplate code. They should be used judiciously, but when applied correctly, they can make your code more expressive, more concise, and more maintainable.
