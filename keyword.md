# Rust Keywords and Symbols

## Keywords

### Variable and Data Declarations

- `let` - Variable binding
- `mut` - Mutability modifier
- `const` - Define constants
- `static` - Define static variables with 'static lifetime

### Data Types and Structures

- `struct` - Define a structure
- `enum` - Define an enumeration
- `union` - Define a union (unsafe)
- `type` - Create type aliases
- `dyn` - Dynamic dispatch to a trait object
- `Self` - Type alias for implementing type
- `self` - Method subject or current module

### Functions and Methods

- `fn` - Define a function
- `async` - Define asynchronous functions/blocks
- `await` - Suspend execution until async operation completes
- `return` - Return from function
- `move` - Force closure to take ownership

### Control Flow

- `if` - Conditional branch
- `else` - Alternative branch in conditional
- `match` - Pattern matching
- `for` - Loop over items from an iterator
- `in` - Part of for loop syntax
- `while` - Conditional loop
- `loop` - Infinite loop
- `break` - Exit a loop
- `continue` - Skip to next iteration of loop

### Modules and Visibility

- `mod` - Define a module
- `pub` - Make items public
- `use` - Import symbols into scope
- `crate` - Refer to the current crate
- `super` - Parent module
- `extern` - Link to external code/crate

### Traits and Implementations

- `trait` - Define a trait
- `impl` - Implement functionality for types
- `where` - Specify type constraints

### Memory Safety and Unsafe Code

- `unsafe` - Denote unsafe code
- `ref` - Bind by reference during pattern matching

### Type Conversions

- `as` - Perform primitive type conversion

### Boolean Literals

- `true` - Boolean true value
- `false` - Boolean false value

### Reserved Keywords (for future use)

- `abstract`, `become`, `box`, `do`, `final`, `macro`
- `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`

## Symbols in Rust

### Operators

#### Arithmetic Operators

- `+` - Addition or unary plus
- `-` - Subtraction or unary negation
- `*` - Multiplication
- `/` - Division
- `%` - Remainder/modulo
- `+=` - Add and assign
- `-=` - Subtract and assign
- `*=` - Multiply and assign
- `/=` - Divide and assign
- `%=` - Remainder and assign

#### Bitwise Operators

- `&` - Bitwise AND
- `|` - Bitwise OR
- `^` - Bitwise XOR
- `!` - Bitwise NOT
- `<<` - Left shift
- `>>` - Right shift
- `&=` - Bitwise AND and assign
- `|=` - Bitwise OR and assign
- `^=` - Bitwise XOR and assign
- `<<=` - Left shift and assign
- `>>=` - Right shift and assign

#### Comparison Operators

- `==` - Equal to
- `!=` - Not equal to
- `>` - Greater than
- `<` - Less than
- `>=` - Greater than or equal to
- `<=` - Less than or equal to

#### Logical Operators

- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

#### Other Operators

- `=` - Assignment
- `=>` - Arrow operator (for match arms, closures)
- `.` - Member access
- `..` - Range (exclusive upper bound)
- `..=` - Range (inclusive upper bound)
- `?` - Error propagation

### Punctuation and Delimiters

- `()` - Parentheses (tuples, function calls, grouping expressions)
- `{}` - Curly braces (code blocks, struct literals)
- `[]` - Square brackets (arrays, indexing)
- `,` - Comma (separates items in lists)
- `;` - Semicolon (statement terminator)
- `:` - Colon (type annotations, trait bounds)
- `::` - Path separator
- `#` - Attribute annotation
- `#!` - Inner attribute annotation
- `'` - Lifetime annotation
- `"` - String literal delimiter
- `_` - Wildcard pattern or ignored binding
- `@` - Pattern binding
- `$` - Macro substitution
- `->` - Function return type indicator

### Special Symbols

- `//` - Line comment
- `/* */` - Block comment
- `///` - Doc comment (outer)
- `//!` - Doc comment (inner)
- `&` - Borrow operator (reference)
- `&mut` - Mutable borrow
- `*` - Dereference operator
- `'static` - Static lifetime
- `?Sized` - Relaxed sized trait bound
- `!` - Never type (functions that don't return)

This classification covers all keywords and commonly used symbols in the Rust programming language, organized by their function and purpose in the language.
