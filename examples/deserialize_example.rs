use yaml_parser::{parse_yaml_to, from_yaml, parse_yaml, YamlDeserialize, YamlValue, yaml_field, yaml_optional_field};

// Simple struct example
#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: i64,
    email: Option<String>,
}

impl YamlDeserialize for Person {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(Person {
            name: yaml_field!(value, "name")?,
            age: yaml_field!(value, "age")?,
            email: yaml_optional_field!(value, "email")?,
        })
    }
}

// Nested struct example
#[derive(Debug, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip: i64,
}

impl YamlDeserialize for Address {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(Address {
            street: yaml_field!(value, "street")?,
            city: yaml_field!(value, "city")?,
            zip: yaml_field!(value, "zip")?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct PersonWithAddress {
    name: String,
    age: i64,
    address: Address,
}

impl YamlDeserialize for PersonWithAddress {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(PersonWithAddress {
            name: yaml_field!(value, "name")?,
            age: yaml_field!(value, "age")?,
            address: yaml_field!(value, "address")?,
        })
    }
}

// Config struct with arrays
#[derive(Debug, PartialEq)]
struct Config {
    debug: bool,
    port: i64,
    allowed_hosts: Vec<String>,
    timeout: Option<f64>,
}

impl YamlDeserialize for Config {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(Config {
            debug: yaml_field!(value, "debug")?,
            port: yaml_field!(value, "port")?,
            allowed_hosts: yaml_field!(value, "allowed_hosts")?,
            timeout: yaml_optional_field!(value, "timeout")?,
        })
    }
}

fn main() {
    // Example 1: Simple struct
    println!("=== Example 1: Simple Person ===");
    let person_yaml = r#"
name: Alice Smith
age: 28
email: alice@example.com
"#;

    match parse_yaml_to::<Person>(person_yaml) {
        Ok(person) => {
            println!("Parsed person: {:?}", person);
            assert_eq!(person.name, "Alice Smith");
            assert_eq!(person.age, 28);
            assert_eq!(person.email, Some("alice@example.com".to_string()));
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Person without email (optional field)
    println!("\n=== Example 2: Person without email ===");
    let person_yaml2 = r#"
name: Bob Jones
age: 35
"#;

    match parse_yaml_to::<Person>(person_yaml2) {
        Ok(person) => {
            println!("Parsed person: {:?}", person);
            assert_eq!(person.name, "Bob Jones");
            assert_eq!(person.age, 35);
            assert_eq!(person.email, None);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Nested structs
    println!("\n=== Example 3: Person with Address ===");
    let person_with_address_yaml = r#"
name: Charlie Brown
age: 42
address:
  street: 123 Main St
  city: Springfield
  zip: 12345
"#;

    match parse_yaml_to::<PersonWithAddress>(person_with_address_yaml) {
        Ok(person) => {
            println!("Parsed person with address: {:?}", person);
            assert_eq!(person.name, "Charlie Brown");
            assert_eq!(person.address.city, "Springfield");
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: Config with arrays
    println!("\n=== Example 4: Configuration ===");
    let config_yaml = r#"
debug: true
port: 8080
allowed_hosts:
  - localhost
  - 127.0.0.1
  - example.com
timeout: 30.5
"#;

    match parse_yaml_to::<Config>(config_yaml) {
        Ok(config) => {
            println!("Parsed config: {:?}", config);
            assert_eq!(config.debug, true);
            assert_eq!(config.port, 8080);
            assert_eq!(config.allowed_hosts.len(), 3);
            assert_eq!(config.timeout, Some(30.5));
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Two-step parsing
    println!("\n=== Example 5: Two-step parsing ===");
    let yaml_value = parse_yaml(person_yaml).unwrap();
    match from_yaml::<Person>(&yaml_value) {
        Ok(person) => {
            println!("Two-step parsed person: {:?}", person);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All examples completed successfully!");
}