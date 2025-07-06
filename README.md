# yaml-parser

A simple, fast YAML parser for Rust with struct deserialization support.

## Features

- 🚀 **Fast parsing** - Custom lexer and parser implementation
- 🏗️ **Struct deserialization** - Parse directly into your structs
- 🔧 **Easy to use** - Simple API with helpful macros
- 📦 **Zero dependencies** - No external crates required
- 🛡️ **Type safe** - Full Rust type system support

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
yaml-parser = "0.1.0"
```

## Usage

### Basic parsing

```rust
use yaml_parser::{parse_yaml, YamlValue};

let yaml = "name: John\nage: 30";
let value = parse_yaml(yaml).unwrap();

match value {
    YamlValue::Object(map) => {
        println!("Name: {:?}", map.get("name"));
        println!("Age: {:?}", map.get("age"));
    }
    _ => println!("Not an object"),
}
```

### Struct deserialization

```rust
use yaml_parser::{parse_yaml_to, YamlDeserialize, yaml_field, yaml_optional_field};

#[derive(Debug)]
struct Person {
    name: String,
    age: i64,
    email: Option<String>,
}

impl YamlDeserialize for Person {
    fn from_yaml(value: &yaml_parser::YamlValue) -> yaml_parser::Result<Self> {
        Ok(Person {
            name: yaml_field!(value, "name")?,
            age: yaml_field!(value, "age")?,
            email: yaml_optional_field!(value, "email")?,
        })
    }
}

let yaml = "name: Alice\nage: 30\nemail: alice@example.com";
let person: Person = parse_yaml_to(yaml).unwrap();
println!("{:?}", person);
```

### Nested structures

```rust
use yaml_parser::{parse_yaml_to, YamlDeserialize, yaml_field};

#[derive(Debug)]
struct Config {
    database: Database,
    server: Server,
}

impl YamlDeserialize for Config {
    fn from_yaml(value: &yaml_parser::YamlValue) -> yaml_parser::Result<Self> {
        Ok(Config {
            database: yaml_field!(value, "database")?,
            server: yaml_field!(value, "server")?,
        })
    }
}

// ... Database and Server implementations
```

## Supported Types

- **Primitives**: `String`, `i64`, `i32`, `f64`, `bool`
- **Collections**: `Vec<T>`, `Option<T>`, `HashMap<String, YamlValue>`
- **Custom structs** implementing `YamlDeserialize`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

