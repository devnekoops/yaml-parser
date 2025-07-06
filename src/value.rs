use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<YamlValue>),
    Object(HashMap<String, YamlValue>),
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_value_equality() {
        assert_eq!(YamlValue::String("test".to_string()), YamlValue::String("test".to_string()));
        assert_eq!(YamlValue::Integer(42), YamlValue::Integer(42));
        assert_eq!(YamlValue::Boolean(true), YamlValue::Boolean(true));
        assert_eq!(YamlValue::Null, YamlValue::Null);
    }

    #[test]
    fn test_yaml_value_clone() {
        let value = YamlValue::String("test".to_string());
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }
}