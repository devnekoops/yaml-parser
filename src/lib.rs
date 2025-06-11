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
    Null
}

#[derive(Debug)]
pub enum YamlError{
    ParseError(String),
    IndentationError(String),
    InvalidValue(String)
}

impl fmt::Display for YamlError{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        match self{
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
    ListItem,
    Indent(usize),
    Newline,
    Comment(String),
    Eof
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line:1,
            column: 1,
        }
    }

    fn is_at_end(&self) -> bool{
        self.position > self.input.len()
    }

    fn current_char(&self) -> char{
        self.input.get(self.position).copied().unwrap()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> char{
        let ch = self.current_char();
        self.position += 1;
        self.column += 1;
        ch
    }

    fn skip_whitespace_except_newline(&mut self){
        while matches!(self.current_char(), ' ' | '\t' | '\r'){
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

    fn measure_indent(&mut self) -> usize{
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

        if self.current_char() != ':' {
            return Ok(None)
        }

        let key = self.input[start_pos..self.position]
                              .iter()
                              .collect::<String>()
                              .trim()
                              .to_string();
        self.advance();
        self.skip_whitespace_except_newline();

        // read value
        let value_start = self.position;
        while !matches!(self.current_char(), '\n' | '\0') {
            self.advance();
        }

        let value = if value_start == self.position{
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
            self.skip_whitespace_except_newline();

            if self.is_at_end(){
                break;
            }

            match self.current_char(){
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                    self.line += 1;
                    self.column += 1;
                }
                '#' => {
                    tokens.push(self.read_comment())
                }
                '-' if self.peek_char() == Some(' ') => {
                    tokens.push(Token::ListItem);
                    self.advance(); // '-'
                    self.advance(); // ' '
                }
                _ => {
                    let indent = self.measure_indent();
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
