//! A simple YAML parser for Rust
//! 
//! This crate provides functionality to parse YAML documents into Rust data structures.
//! It supports both generic parsing to `YamlValue` and direct deserialization to custom structs.
//! 
//! # Features
//! 
//! - Parse YAML into generic `YamlValue` enum
//! - Deserialize directly to custom structs with `YamlDeserialize` trait
//! - Support for nested structures, arrays, and optional fields
//! - Convenient macros for field extraction
//! - Comprehensive error handling
//! 
//! # Basic Usage
//! 
//! ## Generic Parsing
//! 
//! ```rust
//! use yaml_parser::{parse_yaml, YamlValue};
//! 
//! let yaml = "name: John\nage: 30";
//! let value = parse_yaml(yaml).unwrap();
//! 
//! match value {
//!     YamlValue::Object(map) => {
//!         println!("Name: {:?}", map.get("name"));
//!         println!("Age: {:?}", map.get("age"));
//!     }
//!     _ => println!("Not an object"),
//! }
//! ```
//! 
//! ## Struct Deserialization
//! 
//! ```rust
//! use yaml_parser::{parse_yaml_to, YamlDeserialize, yaml_field, yaml_optional_field};
//! 
//! #[derive(Debug)]
//! struct Person {
//!     name: String,
//!     age: i64,
//!     email: Option<String>,
//! }
//! 
//! impl YamlDeserialize for Person {
//!     fn from_yaml(value: &yaml_parser::YamlValue) -> yaml_parser::Result<Self> {
//!         Ok(Person {
//!             name: yaml_field!(value, "name")?,
//!             age: yaml_field!(value, "age")?,
//!             email: yaml_optional_field!(value, "email")?,
//!         })
//!     }
//! }
//! 
//! let yaml = "name: Alice\nage: 30\nemail: alice@example.com";
//! let person: Person = parse_yaml_to(yaml).unwrap();
//! println!("{:?}", person);
//! ```

pub mod error;
pub mod value;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod deserialize;

// Re-export the main types and functions
pub use error::{YamlError, Result};
pub use value::YamlValue;
pub use token::Token;
pub use lexer::Lexer;
pub use parser::Parser;
pub use deserialize::YamlDeserialize;

/// Parse a YAML string into a YamlValue
/// 
/// # Arguments
/// 
/// * `input` - A string slice containing the YAML to parse
/// 
/// # Returns
/// 
/// Returns a `Result<YamlValue>` containing the parsed YAML structure or an error
/// 
/// # Example
/// 
/// ```rust
/// use yaml_parser::parse_yaml;
/// 
/// let yaml = "key: value";
/// let result = parse_yaml(yaml).unwrap();
/// ```
pub fn parse_yaml(input: &str) -> Result<YamlValue> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Parse a YAML string directly into a type that implements YamlDeserialize
/// 
/// # Arguments
/// 
/// * `input` - A string slice containing the YAML to parse
/// 
/// # Returns
/// 
/// Returns a `Result<T>` containing the parsed structure or an error
/// 
/// # Example
/// 
/// ```rust
/// use yaml_parser::{parse_yaml_to, YamlDeserialize};
/// 
/// struct Person {
///     name: String,
///     age: i64,
/// }
/// 
/// impl YamlDeserialize for Person {
///     fn from_yaml(value: &yaml_parser::YamlValue) -> yaml_parser::Result<Self> {
///         Ok(Person {
///             name: yaml_parser::yaml_field!(value, "name")?,
///             age: yaml_parser::yaml_field!(value, "age")?,
///         })
///     }
/// }
/// 
/// let yaml = "name: John\nage: 30";
/// let person: Person = parse_yaml_to(yaml).unwrap();
/// ```
pub fn parse_yaml_to<T: YamlDeserialize>(input: &str) -> Result<T> {
    let yaml_value = parse_yaml(input)?;
    T::from_yaml(&yaml_value)
}

/// Convert a YamlValue to a type that implements YamlDeserialize
/// 
/// # Arguments
/// 
/// * `value` - A YamlValue to convert
/// 
/// # Returns
/// 
/// Returns a `Result<T>` containing the converted structure or an error
/// 
/// # Example
/// 
/// ```rust
/// use yaml_parser::{parse_yaml, from_yaml, YamlDeserialize, YamlValue, yaml_field};
/// 
/// struct Config {
///     debug: bool,
///     port: i64,
/// }
/// 
/// impl YamlDeserialize for Config {
///     fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
///         Ok(Config {
///             debug: yaml_parser::yaml_field!(value, "debug")?,
///             port: yaml_parser::yaml_field!(value, "port")?,
///         })
///     }
/// }
/// 
/// let yaml_value = parse_yaml("debug: true\nport: 8080").unwrap();
/// let config: Config = from_yaml(&yaml_value).unwrap();
/// ```
pub fn from_yaml<T: YamlDeserialize>(value: &YamlValue) -> Result<T> {
    T::from_yaml(value)
}