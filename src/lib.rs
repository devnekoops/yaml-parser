use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
    String(String),
    Integer(i64),
    Fload(f64),
    Boolean(bool),
    Array(Vec<YamlValue>),
    Object(HashMap<String, YamlValue>),
    Null,
}

#[derive(Debug)]
pub enum YamlError {
    ParseError(String),
    IndentationError(String),
    InvalidValue(String),
}

impl fmt::Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YamlError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            YamlError::IndentationError(msg) => write!(f, "Indentation error: {}", msg),
            YamlError::InvalidValue(msg) => write!(f, "Invalid Value error: {}", msg),
        }
    }
}

impl std::error::Error for YamlError {}

pub type Result<T> = std::result::Result<T, YamlError>;

//
//

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Key(String),
    Value(String),
    String(String),
    ListItem,
    Indent(usize),
    Newline,
    Comment(String),
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
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

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let ch = self.current_char();
            self.position += 1;
            self.column += 1;
            ch
        } else {
            self.current_char()
        }
    }

    fn skip_whitespace_except_newline(&mut self) {
        while matches!(self.current_char(), ' ' | '\t' | '\r') {
            self.advance();
        }
    }

    fn read_comment(&mut self) -> Token {
        self.advance(); // '#'
        let mut comment = String::new();

        while !matches!(self.current_char(), '\n' | '\0') {
            comment.push(self.advance());
        }

        Token::Comment(comment.trim().to_string())
    }

    fn measure_indent(&mut self) -> usize {
        let start_pos = self.position;
        while matches!(self.current_char(), ' ' | '\t') {
            self.advance();
        }
        self.position - start_pos
    }

    fn read_key_value(&mut self) -> Result<Option<Token>> {
        let start_pos = self.position;

        // read key
        while !matches!(self.current_char(), ':' | '\n' | '\0') {
            self.advance();
        }

        if start_pos == self.position {
            return Ok(None);
        }

        if self.current_char() != ':' {
            return Ok(Some(Token::String(
                self.input[start_pos..self.position]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_string(),
            )));
        }

        let key = self.input[start_pos..self.position]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();

        self.advance(); // skip ':'
        self.skip_whitespace_except_newline();

        // read value
        let value_start = self.position;
        while !matches!(self.current_char(), '\n' | '\0' | '#') {
            self.advance();
        }

        let value = if value_start == self.position {
            String::new()
        } else {
            self.input[value_start..self.position]
                .iter()
                .collect::<String>()
                .trim()
                .to_string()
        };

        Ok(Some(Token::Key(format!("{} : {}", key, value))))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let indent = self.measure_indent();

            // 空白ではないところまで進む
            self.skip_whitespace_except_newline();

            if self.is_at_end() {
                break;
            }

            // 出た文字によって場合分け
            match self.current_char() {
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                    self.line += 1;
                    self.column += 1;
                }
                '#' => tokens.push(self.read_comment()),
                '-' if self.peek_char() == Some(' ') => {
                    tokens.push(Token::ListItem);
                    self.advance(); // '-'
                    self.advance(); // ' '
                }
                _ => {
                    if indent > 0 {
                        tokens.push(Token::Indent(indent));
                    }

                    if let Some(token) = self.read_key_value()? {
                        tokens.push(token);
                    }
                }
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 親モジュール（lib.rs）のアイテムをインポート

    #[test]
    fn test_is_at_end() {
        let lexer = Lexer::new("");
        assert!(lexer.is_at_end(), "Empty input should be at end");

        let mut lexer = Lexer::new("a");
        assert!(!lexer.is_at_end(), "Should not be at end with input");
        lexer.advance();
        assert!(lexer.is_at_end(), "Should not be at end after one char");

        lexer.advance();
        assert!(lexer.is_at_end(), "Should not be at end after one char");
    }

    #[test]
    fn test_current_char() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.current_char(), 'a');
        lexer.advance();
        assert_eq!(lexer.current_char(), 'b');
        lexer.advance();
        assert_eq!(lexer.current_char(), 'c');
        lexer.advance();
        assert_eq!(lexer.current_char(), '\0');
    }

    #[test]
    fn test_peek_char() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.peek_char(), Some('b'));
        lexer.advance();
        assert_eq!(lexer.peek_char(), Some('c'));
        lexer.advance();
        assert_eq!(lexer.peek_char(), None); // 'c'の次
        lexer.advance(); // 'c'を消費して終端
        assert_eq!(lexer.peek_char(), None);
    }

    #[test]
    fn test_advance() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.advance(), 'a');
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.column, 2);
        assert_eq!(lexer.advance(), 'b');
        assert_eq!(lexer.position, 2);
        assert_eq!(lexer.column, 3);
        assert_eq!(lexer.advance(), 'c');
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.column, 4);

        // 終端だから進まない
        assert_eq!(lexer.advance(), '\0');
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.column, 4);
    }

    #[test]
    fn test_skip_whitespace_except_newline() {
        let mut lexer = Lexer::new("  \t\r  abc");
        lexer.skip_whitespace_except_newline();
        assert_eq!(lexer.position, 6);
        assert_eq!(lexer.current_char(), 'a');

        let mut lexer = Lexer::new("  \nabc");
        lexer.skip_whitespace_except_newline();
        assert_eq!(lexer.position, 2); // newlineで止まる
        assert_eq!(lexer.current_char(), '\n');

        let mut lexer = Lexer::new("");
        lexer.skip_whitespace_except_newline();
        assert_eq!(lexer.position, 0); // 空文字列
    }

    #[test]
    fn test_read_comment() {
        let mut lexer = Lexer::new("# This is a comment\nNext line");
        let token = lexer.read_comment();
        assert_eq!(token, Token::Comment("This is a comment".to_string()));
        assert_eq!(lexer.current_char(), '\n'); // コメントの後の改行で止まる

        let mut lexer = Lexer::new("# Another comment");
        let token = lexer.read_comment();
        assert_eq!(token, Token::Comment("Another comment".to_string()));
        assert!(lexer.is_at_end()); // EOFで止まる
    }

    #[test]
    fn test_measure_indent() {
        let mut lexer = Lexer::new("    key: value");
        let indent = lexer.measure_indent();
        assert_eq!(indent, 4);
        assert_eq!(lexer.current_char(), 'k'); // インデントの次の文字で止まる

        let mut lexer = Lexer::new("\t\tkey: value");
        let indent = lexer.measure_indent();
        assert_eq!(indent, 2);
        assert_eq!(lexer.current_char(), 'k');

        let mut lexer = Lexer::new("key: value");
        let indent = lexer.measure_indent();
        assert_eq!(indent, 0);
        assert_eq!(lexer.current_char(), 'k');

        let mut lexer = Lexer::new("");
        let indent = lexer.measure_indent();
        assert_eq!(indent, 0);
        assert_eq!(lexer.current_char(), '\0');
    }

    #[test]
    fn test_read_key_value_valid() {
        let mut lexer = Lexer::new("key: value\n");
        let token_option = lexer.read_key_value().unwrap();
        assert_eq!(token_option, Some(Token::Key("key : value".to_string())));
        assert_eq!(lexer.current_char(), '\n'); // 改行手前で止まる

        let mut lexer = Lexer::new("  another_key :  another_value  ");
        let token_option = lexer.read_key_value().unwrap();
        assert_eq!(
            token_option,
            Some(Token::Key("another_key : another_value".to_string()))
        );
        assert!(lexer.is_at_end()); // EOFで止まる

        let mut lexer = Lexer::new("empty_value:");
        let token_option = lexer.read_key_value().unwrap();
        assert_eq!(token_option, Some(Token::Key("empty_value : ".to_string())));
        assert!(lexer.is_at_end());
    }

    #[test]
    fn test_read_key_value_no_colon() {
        let mut lexer = Lexer::new("this is not a key value pair\n");
        let result = lexer.read_key_value();
        // 現在のtokenizeロジックでは、read_key_valueがNoneを返すとtokenizeがParseErrorを出す
        // そのため、read_key_valueはNoneを返すことを期待
        assert_eq!(result.unwrap(), None);
        // ここでパーサーの状態が、コロンまで読み進めた状態になることに注意
        // lexer.current_char() は '\n' になっているはず
        assert_eq!(lexer.current_char(), '\n');

        let mut lexer = Lexer::new("just_a_word");
        let result = lexer.read_key_value();
        assert_eq!(result.unwrap(), None);
        assert!(lexer.is_at_end()); // EOFまで読み進む
    }

    #[test]
    fn test_read_key_value_empty_string() {
        let mut lexer = Lexer::new("");
        let token_option = lexer.read_key_value().unwrap();
        assert_eq!(token_option, None);
        assert!(lexer.is_at_end());
    }
}
