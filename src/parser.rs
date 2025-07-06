use std::collections::HashMap;

use crate::error::{Result, YamlError};
use crate::token::Token;
use crate::value::YamlValue;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    #[allow(dead_code)]
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    #[allow(dead_code)]
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }

    #[allow(dead_code)]
    fn consume_if(&mut self, check_fn: impl Fn(&Token) -> bool) -> bool {
        if check_fn(self.peek()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline | Token::Comment(_)) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<YamlValue> {
        self.skip_newlines();
        
        // トップレベルで複数のキーバリューペアがある場合はオブジェクトとして扱う
        if matches!(self.peek(), Token::Key(_)) {
            let mut map = HashMap::new();
            
            while !self.is_at_end() {
                self.skip_newlines();
                
                if self.is_at_end() {
                    break;
                }
                
                // Keyがある場合のみ処理
                if let Token::Key(key) = self.peek() {
                    let key = key.clone();
                    self.advance();
                    
                    // Colonを期待
                    if !matches!(self.peek(), Token::Colon) {
                        return Err(YamlError::ParseError("Expected ':' after key".to_string()));
                    }
                    self.advance();
                    
                    // 値をパース
                    self.skip_newlines();
                    
                    let value = if matches!(self.peek(), Token::Indent(_)) {
                        // ネストした構造
                        self.advance(); // consume indent
                        let nested = self.parse_value()?;
                        
                        // Consume dedent if present
                        if matches!(self.peek(), Token::Dedent(_)) {
                            self.advance();
                        }
                        
                        nested
                    } else if matches!(self.peek(), Token::Newline) {
                        // 空の値の後にネストした内容
                        self.advance();
                        if matches!(self.peek(), Token::Indent(_)) {
                            self.advance();
                            let nested = self.parse_value()?;
                            if matches!(self.peek(), Token::Dedent(_)) {
                                self.advance();
                            }
                            nested
                        } else {
                            YamlValue::Null
                        }
                    } else {
                        // シンプルな値
                        match self.peek() {
                            Token::Value(v) => {
                                let val = v.clone();
                                self.advance();
                                val
                            }
                            Token::ListItem => {
                                self.parse_array()?
                            }
                            _ => YamlValue::Null,
                        }
                    };
                    
                    map.insert(key, value);
                } else {
                    break;
                }
            }
            
            Ok(YamlValue::Object(map))
        } else {
            // 単一の値またはリスト
            let value = self.parse_value()?;
            self.skip_newlines();
            
            // Dedentトークンをスキップ
            while matches!(self.peek(), Token::Dedent(_)) {
                self.advance();
            }
            
            if !self.is_at_end() {
                return Err(YamlError::ParseError("Unexpected content after document".to_string()));
            }
            
            Ok(value)
        }
    }

    fn parse_value(&mut self) -> Result<YamlValue> {
        self.skip_newlines();

        match self.peek() {
            Token::Value(val) => {
                let value = val.clone();
                self.advance();
                Ok(value)
            }
            Token::Key(_) => self.parse_object(),
            Token::ListItem => self.parse_array(),
            Token::Eof => Err(YamlError::UnexpectedEof),
            _ => Err(YamlError::ParseError(format!("Unexpected token: {:?}", self.peek()))),
        }
    }

    fn parse_object(&mut self) -> Result<YamlValue> {
        let mut map = HashMap::new();
        let initial_indent = self.get_current_indent();

        loop {
            self.skip_newlines();

            // Check for dedent or end
            if matches!(self.peek(), Token::Dedent(_) | Token::Eof) {
                break;
            }

            // Check if we're at the same indent level
            let current_indent = self.get_current_indent();
            if current_indent < initial_indent {
                break;
            }

            // Parse key
            let key = match self.peek() {
                Token::Key(k) => {
                    let key = k.clone();
                    self.advance();
                    key
                }
                _ => break,
            };

            // Expect colon
            if !matches!(self.peek(), Token::Colon) {
                return Err(YamlError::ParseError("Expected ':' after key".to_string()));
            }
            self.advance();

            // Parse value
            self.skip_newlines();
            
            let value = if matches!(self.peek(), Token::Indent(_)) {
                // Nested structure
                self.advance(); // consume indent
                let nested = self.parse_value()?;
                
                // Consume dedent if present
                if matches!(self.peek(), Token::Dedent(_)) {
                    self.advance();
                }
                
                nested
            } else if matches!(self.peek(), Token::Newline) {
                // Empty value before nested content
                self.advance();
                if matches!(self.peek(), Token::Indent(_)) {
                    self.advance();
                    let nested = self.parse_value()?;
                    if matches!(self.peek(), Token::Dedent(_)) {
                        self.advance();
                    }
                    nested
                } else {
                    YamlValue::Null
                }
            } else {
                // Simple value
                match self.peek() {
                    Token::Value(v) => {
                        let val = v.clone();
                        self.advance();
                        val
                    }
                    Token::ListItem => {
                        self.parse_array()?
                    }
                    _ => YamlValue::Null,
                }
            };

            map.insert(key, value);
        }

        Ok(YamlValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<YamlValue> {
        let mut array = Vec::new();
        let initial_indent = self.get_current_indent();

        while matches!(self.peek(), Token::ListItem) {
            self.advance(); // consume '-'
            
            // Parse the value after the list item
            let value = if matches!(self.peek(), Token::Newline) {
                // Complex nested structure
                self.advance();
                if matches!(self.peek(), Token::Indent(_)) {
                    self.advance();
                    let nested = self.parse_value()?;
                    if matches!(self.peek(), Token::Dedent(_)) {
                        self.advance();
                    }
                    nested
                } else {
                    YamlValue::Null
                }
            } else {
                // Simple value on same line
                match self.peek() {
                    Token::Value(v) => {
                        let val = v.clone();
                        self.advance();
                        val
                    }
                    Token::Key(_) => {
                        // リストアイテムと同じレベルのオブジェクト
                        let mut map = HashMap::new();
                        let _obj_indent = self.get_current_indent();
                        
                        loop {
                            // keyがあるか確認
                            if let Token::Key(key) = self.peek() {
                                let key = key.clone();
                                self.advance();
                                
                                // colonを期待
                                if !matches!(self.peek(), Token::Colon) {
                                    return Err(YamlError::ParseError("Expected ':' after key".to_string()));
                                }
                                self.advance();
                                
                                // 値をパース
                                let value = match self.peek() {
                                    Token::Value(v) => {
                                        let val = v.clone();
                                        self.advance();
                                        val
                                    }
                                    _ => YamlValue::Null,
                                };
                                
                                map.insert(key, value);
                                
                                // 次の行を確認
                                if matches!(self.peek(), Token::Newline) {
                                    self.advance();
                                    
                                    // インデントトークンをスキップ
                                    if matches!(self.peek(), Token::Indent(_)) {
                                        self.advance();
                                    }
                                    
                                    // Dedentがあれば処理を終了
                                    if matches!(self.peek(), Token::Dedent(_)) {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        
                        YamlValue::Object(map)
                    }
                    _ => YamlValue::Null,
                }
            };

            array.push(value);
            
            self.skip_newlines();
            
            // Dedentトークンをスキップ
            while matches!(self.peek(), Token::Dedent(_)) {
                let dedent_level = if let Token::Dedent(level) = self.peek() {
                    *level
                } else {
                    0
                };
                
                // 配列のレベルより深いDedentのみスキップ
                if dedent_level > initial_indent {
                    self.advance();
                } else {
                    break;
                }
            }
            
            // Check if we're still at the same indent level
            let current_indent = self.get_current_indent();
            if current_indent < initial_indent {
                break;
            }
            
            if matches!(self.peek(), Token::Eof) {
                break;
            }
        }

        Ok(YamlValue::Array(array))
    }

    fn get_current_indent(&self) -> usize {
        let mut indent_level = 0;
        let mut indent_stack = vec![0];
        
        // トークンを最初から現在位置まで走査してインデントレベルを追跡
        for i in 0..self.current {
            match &self.tokens[i] {
                Token::Indent(level) => {
                    indent_stack.push(*level);
                    indent_level = *level;
                }
                Token::Dedent(_) => {
                    if indent_stack.len() > 1 {
                        indent_stack.pop();
                        indent_level = *indent_stack.last().unwrap();
                    }
                }
                _ => {}
            }
        }
        
        indent_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_yaml_helper(input: &str) -> Result<YamlValue> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_parse_simple_object() {
        let yaml = "name: John\nage: 30";
        let result = parse_yaml_helper(yaml).unwrap();
        
        match result {
            YamlValue::Object(map) => {
                assert_eq!(map.get("name"), Some(&YamlValue::String("John".to_string())));
                assert_eq!(map.get("age"), Some(&YamlValue::Integer(30)));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_nested_object() {
        let yaml = "person:\n  name: John\n  age: 30";
        let result = parse_yaml_helper(yaml).unwrap();
        
        match result {
            YamlValue::Object(map) => {
                match map.get("person") {
                    Some(YamlValue::Object(person)) => {
                        assert_eq!(person.get("name"), Some(&YamlValue::String("John".to_string())));
                        assert_eq!(person.get("age"), Some(&YamlValue::Integer(30)));
                    }
                    _ => panic!("Expected nested object"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_array() {
        let yaml = "- apple\n- banana\n- orange";
        let result = parse_yaml_helper(yaml).unwrap();
        
        match result {
            YamlValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], YamlValue::String("apple".to_string()));
                assert_eq!(arr[1], YamlValue::String("banana".to_string()));
                assert_eq!(arr[2], YamlValue::String("orange".to_string()));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_mixed_types() {
        let yaml = "string: hello\ninteger: 42\nfloat: 3.14\nboolean: true\nnull_value: null";
        let result = parse_yaml_helper(yaml).unwrap();
        
        match result {
            YamlValue::Object(map) => {
                assert_eq!(map.get("string"), Some(&YamlValue::String("hello".to_string())));
                assert_eq!(map.get("integer"), Some(&YamlValue::Integer(42)));
                assert!(matches!(map.get("float"), Some(YamlValue::Float(f)) if (f - 3.14).abs() < f64::EPSILON));
                assert_eq!(map.get("boolean"), Some(&YamlValue::Boolean(true)));
                assert_eq!(map.get("null_value"), Some(&YamlValue::Null));
            }
            _ => panic!("Expected object"),
        }
    }
}