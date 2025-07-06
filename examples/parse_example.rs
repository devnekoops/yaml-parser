use yaml_parser::{parse_yaml, YamlValue};

fn main() {
    // 簡単な例
    let simple_yaml = r#"
name: John Doe
age: 30
email: john@example.com
"#;

    match parse_yaml(simple_yaml) {
        Ok(value) => {
            println!("Simple YAML parsed successfully:");
            print_yaml_value(&value, 0);
            println!();
        }
        Err(e) => println!("Error parsing simple YAML: {}", e),
    }

    // ネストした構造の例
    let nested_yaml = r#"
person:
  name: Alice
  age: 25
  address:
    street: 123 Main St
    city: Springfield
    zip: 12345
"#;

    match parse_yaml(nested_yaml) {
        Ok(value) => {
            println!("Nested YAML parsed successfully:");
            print_yaml_value(&value, 0);
            println!();
        }
        Err(e) => println!("Error parsing nested YAML: {}", e),
    }

    // 配列を含む例
    let array_yaml = r#"
fruits:
  - apple
  - banana
  - orange
numbers:
  - 1
  - 2
  - 3
"#;

    match parse_yaml(array_yaml) {
        Ok(value) => {
            println!("Array YAML parsed successfully:");
            print_yaml_value(&value, 0);
            println!();
        }
        Err(e) => println!("Error parsing array YAML: {}", e),
    }

    // 複雑な構造の例
    let complex_yaml = r#"
server:
  host: localhost
  port: 8080
  ssl: true
database:
  type: postgresql
  connection:
    host: db.example.com
    port: 5432
    username: admin
    password: "secret123"
features:
  - authentication
  - caching
  - logging
users:
  - name: Alice
    role: admin
    active: true
  - name: Bob
    role: user
    active: false
"#;

    match parse_yaml(complex_yaml) {
        Ok(value) => {
            println!("Complex YAML parsed successfully:");
            print_yaml_value(&value, 0);
            
            // 特定の値へのアクセス例
            if let YamlValue::Object(map) = &value {
                if let Some(YamlValue::Object(server)) = map.get("server") {
                    if let Some(YamlValue::Integer(port)) = server.get("port") {
                        println!("\nServer port: {}", port);
                    }
                }
                
                if let Some(YamlValue::Array(features)) = map.get("features") {
                    println!("\nFeatures:");
                    for feature in features {
                        if let YamlValue::String(f) = feature {
                            println!("  - {}", f);
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error parsing complex YAML: {}", e),
    }
}

// YAMLの値を整形して表示するヘルパー関数
fn print_yaml_value(value: &YamlValue, indent: usize) {
    let indent_str = "  ".repeat(indent);
    
    match value {
        YamlValue::String(s) => println!("{}\"{}\"", indent_str, s),
        YamlValue::Integer(i) => println!("{}{}", indent_str, i),
        YamlValue::Float(f) => println!("{}{}", indent_str, f),
        YamlValue::Boolean(b) => println!("{}{}", indent_str, b),
        YamlValue::Null => println!("{}null", indent_str),
        YamlValue::Array(arr) => {
            println!("{}[", indent_str);
            for item in arr {
                print_yaml_value(item, indent + 1);
            }
            println!("{}]", indent_str);
        }
        YamlValue::Object(map) => {
            println!("{}{}", indent_str, "{");
            for (key, val) in map {
                print!("{}{}: ", "  ".repeat(indent + 1), key);
                if matches!(val, YamlValue::Object(_) | YamlValue::Array(_)) {
                    println!();
                    print_yaml_value(val, indent + 1);
                } else {
                    print_yaml_value(val, 0);
                }
            }
            println!("{}{}", indent_str, "}");
        }
    }
}