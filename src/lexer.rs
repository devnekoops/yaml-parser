use crate::error::{Result, YamlError};
use crate::token::Token;
use crate::value::YamlValue;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,  // インデントレベルのスタック
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0], // 初期インデントレベルは0
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn current_char(&self) -> char {
        self.input.get(self.position).copied().unwrap_or('\0')
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    #[allow(dead_code)]
    fn peek_n(&self, n: usize) -> Option<char> {
        self.input.get(self.position + n).copied()
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let ch = self.current_char();
            self.position += 1;
            
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            ch
        } else {
            '\0'
        }
    }

    fn skip_whitespace_except_newline(&mut self) {
        while matches!(self.current_char(), ' ' | '\t' | '\r') {
            self.advance();
        }
    }

    #[allow(dead_code)]
    fn skip_whitespace(&mut self) {
        while matches!(self.current_char(), ' ' | '\t' | '\r' | '\n') {
            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
    }

    // インデント測定と DEDENTトークンの生成
    fn handle_indentation(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut indent_level = 0;
        
        // 行の開始時のインデントを測定
        while matches!(self.current_char(), ' ' | '\t') {
            self.advance();
            indent_level += 1;
        }
        
        // 空行やコメント行の場合はインデント処理をスキップ
        if matches!(self.current_char(), '\n' | '#' | '\0') {
            return Ok(tokens);
        }
        
        let current_indent = *self.indent_stack.last().unwrap();
        
        if indent_level > current_indent {
            // インデント増加
            self.indent_stack.push(indent_level);
            tokens.push(Token::Indent(indent_level));
        } else if indent_level < current_indent {
            // インデント減少 - 複数レベル戻る可能性がある
            while let Some(&stack_indent) = self.indent_stack.last() {
                if stack_indent <= indent_level {
                    break;
                }
                self.indent_stack.pop();
                tokens.push(Token::Dedent(stack_indent));
            }
            
            // 不正なインデントレベルの検出
            if self.indent_stack.last() != Some(&indent_level) {
                return Err(YamlError::IndentationError(
                    format!("Invalid indentation level {} at line {}", indent_level, self.line)
                ));
            }
        }
        
        Ok(tokens)
    }

    fn read_comment(&mut self) -> Token {
        self.advance(); // '#'をスキップ
        let mut comment = String::new();

        while !matches!(self.current_char(), '\n' | '\0') {
            comment.push(self.advance());
        }

        Token::Comment(comment.trim().to_string())
    }

    // 値を読み取って適切な型に変換
    fn read_value(&mut self) -> Result<YamlValue> {
        self.skip_whitespace_except_newline();
        
        if self.is_at_end() || matches!(self.current_char(), '\n' | '#') {
            return Ok(YamlValue::Null);
        }
        
        let start_pos = self.position;
        
        // 引用符付き文字列の処理
        if matches!(self.current_char(), '"' | '\'') {
            return self.read_quoted_string();
        }
        
        // 通常の値を読み取り
        while !matches!(self.current_char(), '\n' | '\0' | '#') {
            self.advance();
        }
        
        let value_str = self.input[start_pos..self.position]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
            
        self.parse_scalar_value(&value_str)
    }

    fn read_quoted_string(&mut self) -> Result<YamlValue> {
        let quote_char = self.advance(); // " or '
        let mut value = String::new();
        
        while !self.is_at_end() && self.current_char() != quote_char {
            let ch = self.advance();
            if ch == '\\' && quote_char == '"' {
                // エスケープシーケンスの処理（ダブルクォートのみ）
                match self.current_char() {
                    'n' => { self.advance(); value.push('\n'); }
                    't' => { self.advance(); value.push('\t'); }
                    'r' => { self.advance(); value.push('\r'); }
                    '\\' => { self.advance(); value.push('\\'); }
                    '"' => { self.advance(); value.push('"'); }
                    _ => {
                        value.push(ch);
                        value.push(self.advance());
                    }
                }
            } else {
                value.push(ch);
            }
        }
        
        if self.current_char() == quote_char {
            self.advance(); // 終了クォートをスキップ
        } else {
            return Err(YamlError::UnexpectedEof);
        }
        
        Ok(YamlValue::String(value))
    }

    fn parse_scalar_value(&self, value: &str) -> Result<YamlValue> {
        match value {
            "true" | "True" | "TRUE" => Ok(YamlValue::Boolean(true)),
            "false" | "False" | "FALSE" => Ok(YamlValue::Boolean(false)),
            "null" | "Null" | "NULL" | "~" | "" => Ok(YamlValue::Null),
            _ => {
                // 数値の解析を試行
                if let Ok(int_val) = value.parse::<i64>() {
                    Ok(YamlValue::Integer(int_val))
                } else if let Ok(float_val) = value.parse::<f64>() {
                    Ok(YamlValue::Float(float_val))
                } else {
                    Ok(YamlValue::String(value.to_string()))
                }
            }
        }
    }

    fn read_key(&mut self) -> Result<String> {
        let start_pos = self.position;
        
        // キーの読み取り（コロンまで）
        while !matches!(self.current_char(), ':' | '\n' | '\0') {
            self.advance();
        }
        
        if start_pos == self.position {
            return Err(YamlError::ParseError("Empty key".to_string()));
        }
        
        let key = self.input[start_pos..self.position]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
            
        if key.is_empty() {
            return Err(YamlError::ParseError("Empty key after trimming".to_string()));
        }
        
        Ok(key)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        if self.is_at_end() {
            return Ok(Some(Token::Eof));
        }

        match self.current_char() {
            '\n' => {
                self.advance();
                Ok(Some(Token::Newline))
            }
            '#' => Ok(Some(self.read_comment())),
            ':' => {
                self.advance();
                Ok(Some(Token::Colon))
            }
            '-' => {
                if self.peek_char() == Some(' ') || self.peek_char() == Some('\n') {
                    self.advance(); // '-'
                    if self.current_char() == ' ' {
                        self.advance(); // ' '
                    }
                    Ok(Some(Token::ListItem))
                } else {
                    // ダッシュで始まる値として扱う
                    let value = self.read_value()?;
                    Ok(Some(Token::Value(value)))
                }
            }
            _ => {
                // キーまたは値の読み取り
                let _start_pos = self.position;
                
                // コロンがあるかチェック
                let mut temp_pos = self.position;
                let mut found_colon = false;
                
                while temp_pos < self.input.len() {
                    match self.input[temp_pos] {
                        ':' => {
                            // コロンの後にスペースまたは改行があるかチェック
                            if temp_pos + 1 < self.input.len() {
                                match self.input[temp_pos + 1] {
                                    ' ' | '\t' | '\n' | '\r' => {
                                        found_colon = true;
                                        break;
                                    }
                                    _ => {}
                                }
                            } else {
                                found_colon = true;
                                break;
                            }
                        }
                        '\n' | '#' => break,
                        _ => {}
                    }
                    temp_pos += 1;
                }
                
                if found_colon {
                    // キーとして読み取り
                    let key = self.read_key()?;
                    Ok(Some(Token::Key(key)))
                } else {
                    // 値として読み取り
                    let value = self.read_value()?;
                    Ok(Some(Token::Value(value)))
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut at_line_start = true;

        while !self.is_at_end() {
            // 行の開始時にインデント処理
            if at_line_start {
                let indent_tokens = self.handle_indentation()?;
                tokens.extend(indent_tokens);
                at_line_start = false;
            }
            
            // 空白をスキップ（改行以外）
            self.skip_whitespace_except_newline();
            
            if self.is_at_end() {
                break;
            }
            
            // 次のトークンを取得
            if let Some(token) = self.next_token()? {
                let is_newline = matches!(token, Token::Newline);
                tokens.push(token);
                
                if is_newline {
                    at_line_start = true;
                }
            }
        }
        
        // 残りのDEDENTトークンを生成
        while self.indent_stack.len() > 1 {
            let indent_level = self.indent_stack.pop().unwrap();
            tokens.push(Token::Dedent(indent_level));
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_key_value() {
        let mut lexer = Lexer::new("key: value");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Key("key".to_string()),
            Token::Colon,
            Token::Value(YamlValue::String("value".to_string())),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_indented_structure() {
        let mut lexer = Lexer::new("parent:\n  child: value");
        let tokens = lexer.tokenize().unwrap();
        
        // 期待されるトークン構造をテスト
        assert!(tokens.contains(&Token::Key("parent".to_string())));
        assert!(tokens.contains(&Token::Indent(2)));
        assert!(tokens.contains(&Token::Key("child".to_string())));
    }

    #[test]
    fn test_list_items() {
        let mut lexer = Lexer::new("- item1\n- item2");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.contains(&Token::ListItem));
        assert!(tokens.contains(&Token::Value(YamlValue::String("item1".to_string()))));
        assert!(tokens.contains(&Token::Value(YamlValue::String("item2".to_string()))));
    }

    #[test]
    fn test_value_types() {
        let mut lexer = Lexer::new("int: 42\nfloat: 3.14\nbool: true\nnull: null");
        let tokens = lexer.tokenize().unwrap();
        
        // 各値の型が正しく判定されることを確認
        assert!(tokens.iter().any(|t| matches!(t, Token::Value(YamlValue::Integer(42)))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Value(YamlValue::Float(f)) if (f - 3.14).abs() < f64::EPSILON)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Value(YamlValue::Boolean(true)))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Value(YamlValue::Null))));
    }

    #[test]
    fn test_quoted_strings() {
        let mut lexer = Lexer::new(r#"quoted: "hello world""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.iter().any(|t| matches!(t, Token::Value(YamlValue::String(s)) if s == "hello world")));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("key: value # this is a comment");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.iter().any(|t| matches!(t, Token::Comment(c) if c == "this is a comment")));
    }
}