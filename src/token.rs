use crate::value::YamlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 基本的なトークン
    Key(String),
    Colon,
    Value(YamlValue),
    
    // 構造トークン
    ListItem,           // -
    BlockSequence,      // - (ブロックシーケンス用)
    
    // インデントとレイアウト
    Indent(usize),
    Dedent(usize),      // インデント減少
    Newline,
    
    // 特殊
    Comment(String),
    Eof,
    
    // 将来の拡張用
    FlowStart,          // [, {
    FlowEnd,            // ], }
    FlowSeparator,      // ,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_equality() {
        assert_eq!(Token::Key("test".to_string()), Token::Key("test".to_string()));
        assert_eq!(Token::Colon, Token::Colon);
        assert_eq!(Token::Indent(2), Token::Indent(2));
        assert_ne!(Token::Indent(2), Token::Indent(4));
    }

    #[test]
    fn test_token_clone() {
        let token = Token::Key("test".to_string());
        let cloned = token.clone();
        assert_eq!(token, cloned);
    }
}