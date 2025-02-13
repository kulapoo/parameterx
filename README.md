# Rust Parameters Library

A flexible and type-safe parameter management system for Rust applications. This library provides multiple ways to store and retrieve typed values using a key-value structure, with support for custom types and various initialization patterns.

## Features

- Type-safe parameter storage and retrieval
- Multiple initialization patterns (direct, builder, macro)
- Support for custom types
- Vector type support through `IntVec`
- String conversion capabilities
- Zero-cost abstractions with Rust's type system

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
parameters = "0.1.0"  # Replace with actual version
```

## Usage

### Basic Usage

The simplest way to use the Parameters system is through direct insertion and retrieval:

```rust
let mut params = Parameters::new();
params.insert("name", "Alice");
params.insert("age", 30);
params.insert("height", 5.9f64);

// Type-safe retrieval
let name: Option<&str> = params.get("name");
let age: Option<i32> = params.get("age");
```

### Builder Pattern

For more complex initialization scenarios, use the builder pattern:

```rust
let params = ParametersBuilder::new()
    .add("name", "Bob")
    .add("scores", IntVec::<i32>(vec![85, 92, 78]))
    .build();
```

### Macro Usage

The library provides a convenient macro for quick parameter creation:

```rust
let params = parameters! {
    "name" => "Charlie",
    "age" => "25",
};
```

### Custom Types

The system supports custom types that implement the necessary traits:

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.age)
    }
}

let person = Person {
    name: "Dave".to_string(),
    age: 35,
};

let params = Parameters::new().with("person", person);
```

## API Reference

### Parameters

- `new()`: Creates a new empty Parameters instance
- `insert<T>(key: &str, value: T)`: Inserts a value with the given key
- `get<T>(key: &str) -> Option<&T>`: Retrieves a value by key with type checking
- `get_string(key: &str) -> Option<String>`: Retrieves a value as a String

### ParametersBuilder

- `new()`: Creates a new builder instance
- `add<T>(key: &str, value: T)`: Adds a parameter to the builder
- `build()`: Constructs the final Parameters instance

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.