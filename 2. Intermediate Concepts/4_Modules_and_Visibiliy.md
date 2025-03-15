# Modules and Visibility in Rust: Code Organization

Rust's module system helps you organize code, control visibility, and manage dependencies. It's a powerful tool for creating maintainable and reusable code.

## Modules Basics

Modules are Rust's way of organizing code into logical units. They help:

1. **Organize code**: Group related functionality
2. **Control visibility**: Determine what's public or private
3. **Manage namespaces**: Prevent name conflicts

### Creating Modules

```rust
// In main.rs or lib.rs
mod physics; // Loads from physics.rs or physics/mod.rs
mod graphics {
    // Inline module
    pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
        // Implementation
    }

    // Nested module
    pub mod colors {
        pub const RED: (u8, u8, u8) = (255, 0, 0);
        pub const GREEN: (u8, u8, u8) = (0, 255, 0);
        pub const BLUE: (u8, u8, u8) = (0, 0, 255);
    }
}
```

### Module File Structure

There are two ways to organize modules in files:

#### 1. Single File Per Module

```
src/
├── main.rs
├── physics.rs
└── graphics.rs
```

In `main.rs`:

```rust
mod physics; // Loads from physics.rs
mod graphics; // Loads from graphics.rs

fn main() {
    physics::simulate();
    graphics::draw_line(0, 0, 100, 100);
}
```

#### 2. Directory Per Module

```
src/
├── main.rs
├── physics/
│   ├── mod.rs
│   ├── forces.rs
│   └── motion.rs
└── graphics/
    ├── mod.rs
    ├── rendering.rs
    └── colors.rs
```

In `physics/mod.rs`:

```rust
pub mod forces; // Loads from physics/forces.rs
pub mod motion; // Loads from physics/motion.rs

pub fn simulate() {
    forces::apply_gravity();
    motion::update_position();
}
```

## Visibility and Privacy

By default, everything in Rust is private. You need to use the `pub` keyword to make items accessible from outside their module.

### Visibility Rules

- **Private (default)**: Only accessible within the current module and its descendants
- **Public (`pub`)**: Accessible from outside the current module
- **Restricted**: Using `pub(crate)`, `pub(super)`, or `pub(in path)` for fine-grained control

```rust
mod audio {
    // Private function - only accessible within the audio module
    fn init_device() {
        // Implementation
    }

    // Public function - accessible from outside the audio module
    pub fn play_sound(filename: &str) {
        init_device(); // Can access private functions
        // Implementation
    }

    // Public struct with private fields
    pub struct Volume {
        level: f32, // Private field
    }

    impl Volume {
        // Public constructor
        pub fn new(level: f32) -> Volume {
            Volume { level }
        }

        // Public getter
        pub fn level(&self) -> f32 {
            self.level
        }
    }

    // Module visible only within the crate
    pub(crate) mod formats {
        // Implementation
    }

    // Module visible only to parent
    pub(super) mod drivers {
        // Implementation
    }
}
```

### Using Items from Modules

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    // Using the fully qualified path
    let sum = math::add(10, 5);

    // Using a shorter path with `use`
    use math::subtract;
    let difference = subtract(10, 5);
}
```

## The `use` Keyword

The `use` keyword brings items into scope, similar to imports in other languages.

### Basic Usage

```rust
// Importing a specific function
use std::collections::HashMap;

// Importing multiple items from the same module
use std::io::{self, Read, Write};

// Importing everything (not usually recommended)
use std::collections::*;

// Renaming with `as`
use std::io::Error as IoError;
```

### Idiomatic `use` Patterns

```rust
// For functions: Import the parent module
use std::io;
fn read_file() -> io::Result<String> {
    // Implementation
}

// For types (structs, enums): Import the type directly
use std::collections::HashMap;
fn count_words(text: &str) -> HashMap<String, usize> {
    // Implementation
}
```

## Organizing a Project with Modules

Let's look at a more comprehensive example of how to structure a project:

```
my_game/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── engine/
    │   ├── mod.rs
    │   ├── physics.rs
    │   └── rendering.rs
    ├── audio/
    │   ├── mod.rs
    │   ├── playback.rs
    │   └── formats/
    │       ├── mod.rs
    │       ├── mp3.rs
    │       └── wav.rs
    └── ui/
        ├── mod.rs
        ├── widgets.rs
        └── layout.rs
```

In `lib.rs`:

```rust
pub mod engine;
pub mod audio;
pub mod ui;

// Re-export common types for easier usage
pub use engine::Game;
pub use ui::widgets::Button;
```

In `main.rs`:

```rust
use my_game::Game;
use my_game::audio::playback;

fn main() {
    let mut game = Game::new();
    playback::init();
    game.run();
}
```

## The `pub use` Pattern (Re-exporting)

Re-exporting is a powerful pattern for creating a clean public API:

```rust
// In lib.rs
mod data_processing;
mod storage;

// Re-export specific items for a cleaner API
pub use data_processing::process;
pub use storage::save;
```

Now users can do `use my_crate::process` instead of `use my_crate::data_processing::process`.

## Visibility of Struct Fields and Enum Variants

```rust
pub struct Person {
    pub name: String,     // Public field
    pub age: u8,          // Public field
    ssn: String,          // Private field
}

pub enum Color {
    Red,    // Public by default if enum is public
    Green,  // Public by default if enum is public
    Blue,   // Public by default if enum is public
}

pub enum ApiResult {
    pub Success(String),          // Explicit public variant
    pub Error(String),            // Explicit public variant
    InternalError(String),        // Private variant
}
```

## The `mod.rs` vs Flat Approach

There are two common ways to organize modules:

### 1. Traditional (with `mod.rs`)

```
src/
└── audio/
    ├── mod.rs       <- Defines the audio module
    ├── playback.rs
    └── formats.rs
```

### 2. Flat (newer approach)

```
src/
├── audio.rs         <- Defines the audio module
└── audio/
    ├── playback.rs
    └── formats.rs
```

In `audio.rs`:

```rust
pub mod playback;
pub mod formats;
```

The flat approach avoids having multiple files named `mod.rs`, which can be confusing in some editors.

## Best Practices

1. **Keep modules focused**: Each module should have a single responsibility
2. **Control your public API**: Only make items public that users need
3. **Prefer composition over inheritance**: Use modules to compose functionality
4. **Use descriptive names**: Module names should clearly indicate their purpose
