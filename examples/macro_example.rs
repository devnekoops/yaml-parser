use yaml_parser::{parse_yaml_to, YamlDeserialize, YamlValue, yaml_field, yaml_optional_field};

// Simplified struct creation with macro  
macro_rules! yaml_struct {
    (
        $(#[$attr:meta])*
        struct $name:ident {
            $(
                $field:ident: $field_ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        struct $name {
            $(
                $field: $field_ty,
            )*
        }

        impl YamlDeserialize for $name {
            fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
                yaml_struct!(@impl_field value, $name, $($field: $field_ty),*)
            }
        }
    };
    
    // Helper macro for implementation
    (@impl_field $value:expr, $name:ident, $($field:ident: $field_ty:ty),*) => {
        Ok($name {
            $(
                $field: yaml_struct!(@extract_field $value, stringify!($field), $field_ty)?,
            )*
        })
    };
    
    // Extract field - check if it's Option<T>
    (@extract_field $value:expr, $field_name:expr, Option<$inner:ty>) => {
        yaml_optional_field!($value, $field_name)
    };
    
    (@extract_field $value:expr, $field_name:expr, $field_ty:ty) => {
        yaml_field!($value, $field_name)
    };
}

// Using the macro for simpler struct definitions
yaml_struct! {
    #[derive(Debug, PartialEq)]
    struct SimpleConfig {
        host: String,
        port: i64,
        debug: bool,
    }
}

#[derive(Debug, PartialEq)]
struct Database {
    url: String,
    timeout: Option<f64>,
    max_connections: Option<i64>,
}

impl YamlDeserialize for Database {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(Database {
            url: yaml_field!(value, "url")?,
            timeout: yaml_optional_field!(value, "timeout")?,
            max_connections: yaml_optional_field!(value, "max_connections")?,
        })
    }
}

// Complex struct with manual implementation
#[derive(Debug, PartialEq)]
struct ComplexConfig {
    database: Database,
    server: SimpleConfig,
    features: Vec<String>,
}

impl YamlDeserialize for ComplexConfig {
    fn from_yaml(value: &YamlValue) -> yaml_parser::Result<Self> {
        Ok(ComplexConfig {
            database: yaml_field!(value, "database")?,
            server: yaml_field!(value, "server")?,
            features: yaml_field!(value, "features")?,
        })
    }
}

fn main() {
    println!("=== Macro Example: Simple Config ===");
    let simple_yaml = r#"
host: localhost
port: 3000
debug: true
"#;

    match parse_yaml_to::<SimpleConfig>(simple_yaml) {
        Ok(config) => {
            println!("Simple config: {:?}", config);
            assert_eq!(config.host, "localhost");
            assert_eq!(config.port, 3000);
            assert_eq!(config.debug, true);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Macro Example: Database with Optional Fields ===");
    let db_yaml = r#"
url: postgresql://localhost:5432/mydb
timeout: 10.5
"#;

    match parse_yaml_to::<Database>(db_yaml) {
        Ok(db) => {
            println!("Database config: {:?}", db);
            assert_eq!(db.url, "postgresql://localhost:5432/mydb");
            assert_eq!(db.timeout, Some(10.5));
            assert_eq!(db.max_connections, None);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Complex Nested Config ===");
    let complex_yaml = r#"
database:
  url: postgresql://db.example.com:5432/prod
  timeout: 15.0
  max_connections: 100

server:
  host: 0.0.0.0
  port: 8080
  debug: false

features:
  - authentication
  - rate_limiting
  - caching
  - metrics
"#;

    match parse_yaml_to::<ComplexConfig>(complex_yaml) {
        Ok(config) => {
            println!("Complex config: {:?}", config);
            
            // Assertions
            assert_eq!(config.database.url, "postgresql://db.example.com:5432/prod");
            assert_eq!(config.database.timeout, Some(15.0));
            assert_eq!(config.database.max_connections, Some(100));
            
            assert_eq!(config.server.host, "0.0.0.0");
            assert_eq!(config.server.port, 8080);
            assert_eq!(config.server.debug, false);
            
            assert_eq!(config.features.len(), 4);
            assert!(config.features.contains(&"authentication".to_string()));
            assert!(config.features.contains(&"rate_limiting".to_string()));
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All macro examples completed successfully!");
}