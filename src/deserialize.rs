use crate::error::{Result, YamlError};
use crate::value::YamlValue;
use std::collections::HashMap;

/// Trait for types that can be deserialized from YAML
pub trait YamlDeserialize: Sized {
    /// Deserialize from a YamlValue
    fn from_yaml(value: &YamlValue) -> Result<Self>;
}

/// Helper trait for field extraction
pub trait FromYamlField {
    fn from_yaml_field(value: &YamlValue, field_name: &str) -> Result<Self>
    where
        Self: Sized;
}

// Implement YamlDeserialize for primitive types
impl YamlDeserialize for String {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::String(s) => Ok(s.clone()),
            _ => Err(YamlError::InvalidValue(format!("Expected string, found {:?}", value))),
        }
    }
}

impl YamlDeserialize for i64 {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Integer(i) => Ok(*i),
            _ => Err(YamlError::InvalidValue(format!("Expected integer, found {:?}", value))),
        }
    }
}

impl YamlDeserialize for i32 {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Integer(i) => Ok(*i as i32),
            _ => Err(YamlError::InvalidValue(format!("Expected integer, found {:?}", value))),
        }
    }
}

impl YamlDeserialize for f64 {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Float(f) => Ok(*f),
            YamlValue::Integer(i) => Ok(*i as f64),
            _ => Err(YamlError::InvalidValue(format!("Expected float, found {:?}", value))),
        }
    }
}

impl YamlDeserialize for bool {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Boolean(b) => Ok(*b),
            _ => Err(YamlError::InvalidValue(format!("Expected boolean, found {:?}", value))),
        }
    }
}

impl<T: YamlDeserialize> YamlDeserialize for Vec<T> {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    result.push(T::from_yaml(item)?);
                }
                Ok(result)
            }
            _ => Err(YamlError::InvalidValue(format!("Expected array, found {:?}", value))),
        }
    }
}

impl<T: YamlDeserialize> YamlDeserialize for Option<T> {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Null => Ok(None),
            other => Ok(Some(T::from_yaml(other)?)),
        }
    }
}

impl YamlDeserialize for HashMap<String, YamlValue> {
    fn from_yaml(value: &YamlValue) -> Result<Self> {
        match value {
            YamlValue::Object(map) => Ok(map.clone()),
            _ => Err(YamlError::InvalidValue(format!("Expected object, found {:?}", value))),
        }
    }
}

/// Extract a field from a YAML object
pub fn extract_field<T: YamlDeserialize>(value: &YamlValue, field_name: &str) -> Result<T> {
    match value {
        YamlValue::Object(map) => {
            match map.get(field_name) {
                Some(field_value) => T::from_yaml(field_value),
                None => Err(YamlError::InvalidValue(format!("Missing field: {}", field_name))),
            }
        }
        _ => Err(YamlError::InvalidValue(format!("Expected object to extract field {}", field_name))),
    }
}

/// Extract an optional field from a YAML object
pub fn extract_optional_field<T: YamlDeserialize>(value: &YamlValue, field_name: &str) -> Result<Option<T>> {
    match value {
        YamlValue::Object(map) => {
            match map.get(field_name) {
                Some(field_value) => Ok(Some(T::from_yaml(field_value)?)),
                None => Ok(None), // Missing field is Ok for Option
            }
        }
        _ => Err(YamlError::InvalidValue(format!("Expected object to extract field {}", field_name))),
    }
}

// Macro to make field extraction easier
#[macro_export]
macro_rules! yaml_field {
    ($value:expr, $field:expr) => {
        $crate::deserialize::extract_field($value, $field)
    };
}

#[macro_export]
macro_rules! yaml_optional_field {
    ($value:expr, $field:expr) => {
        $crate::deserialize::extract_optional_field($value, $field)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_deserialization() {
        let value = YamlValue::String("hello".to_string());
        let result: String = YamlDeserialize::from_yaml(&value).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_integer_deserialization() {
        let value = YamlValue::Integer(42);
        let result: i64 = YamlDeserialize::from_yaml(&value).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_array_deserialization() {
        let value = YamlValue::Array(vec![
            YamlValue::Integer(1),
            YamlValue::Integer(2),
            YamlValue::Integer(3),
        ]);
        let result: Vec<i64> = YamlDeserialize::from_yaml(&value).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_option_deserialization() {
        let value = YamlValue::Null;
        let result: Option<String> = YamlDeserialize::from_yaml(&value).unwrap();
        assert_eq!(result, None);

        let value = YamlValue::String("test".to_string());
        let result: Option<String> = YamlDeserialize::from_yaml(&value).unwrap();
        assert_eq!(result, Some("test".to_string()));
    }
}