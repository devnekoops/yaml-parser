// テスト対象のクレート（ここでは 'yaml_parser'）をインポートします。
// Cargo.toml の package.name が 'yaml-parser' の場合、クレート名は 'yaml_parser' になります。
use yaml_parser::{Lexer, Result, Token, YamlError, YamlValue};

// tokenizeのヘルパー関数 (Resultを返すように調整)
fn tokenize_string(input: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

#[test]
fn test_tokenize_empty_input() {
    let tokens = tokenize_string("").unwrap();
    assert_eq!(tokens, vec![Token::Eof]);
}

#[test]
fn test_tokenize_basic_key_value() {
    let yaml = "name: Alice";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Key("name".to_string()),
            Token::Colon,
            Token::Value(YamlValue::String("Alice".to_string())),
            Token::Eof
        ]
    );
}

#[test]
fn test_tokenize_multiple_key_values_newline() {
    let yaml = "name: Alice\nage: 30";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Key("name : Alice".to_string()),
            Token::Newline,
            Token::Key("age : 30".to_string()),
            Token::Eof
        ]
    );
}

#[test]
fn test_tokenize_nested_structure_with_indent() {
    let yaml = "user:\n  name: Bob";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Key("user : ".to_string()), // キーのみの行
            Token::Newline,
            Token::Indent(2),
            Token::Key("name : Bob".to_string()),
            Token::Eof
        ]
    );
}



/// Data Type Handling

#[test]
fn test_tokenize_different_data_types_in_key_value() {
    let yaml = "string: Hello\ninteger: 123\nfloat: 45.67\nboolean: true\nnull_val: ~";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Key("string : Hello".to_string()),
            Token::Newline,
            Token::Key("integer : 123".to_string()),
            Token::Newline,
            Token::Key("float : 45.67".to_string()),
            Token::Newline,
            Token::Key("boolean : true".to_string()),
            Token::Newline,
            Token::Key("null_val : ~".to_string()),
            Token::Eof
        ]
    );
}

/// Special Characters & Formatting

#[test]
fn test_tokenize_comments() {
    let yaml = "# This is a comment\nkey: value # inline comment";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Comment("This is a comment".to_string()),
            Token::Newline,
            Token::Key("key : value".to_string()),
            Token::Comment("inline comment".to_string()),
            Token::Eof
        ]
    );
}

#[test]
fn test_tokenize_empty_line() {
    let yaml = "key1: value1\n\nkey2: value2";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Key("key1 : value1".to_string()),
            Token::Newline,
            Token::Newline,
            Token::Key("key2 : value2".to_string()),
            Token::Eof
        ]
    );
}

#[test]
fn test_tokenize_leading_and_trailing_whitespace_on_line() {
    let yaml = "  key: value  \n key2: value2 ";
    let tokens = tokenize_string(yaml).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Indent(2),
            Token::Key("key : value".to_string()),
            Token::Newline,
            Token::Indent(1),
            Token::Key("key2 : value2".to_string()),
            Token::Eof
        ]
    );
}

/// Error Handling Scenarios

#[test]
fn test_tokenize_indentation_only_line_error() {
    // インデントの後にキーバリューペアがない行は、Lexerの現在の実装ではParseErrorを返す
    let yaml = "level1: data\n    \nlevel2: more_data";
    let result = tokenize_string(yaml);
    dbg!(&result);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(format!("{}", e).contains("Invalid line format"));
    }
}

#[test]
fn test_tokenize_invalid_line_format_error() {
    let yaml = "key1: value1\ninvalid line without colon\nkey2: value2";
    let result = tokenize_string(yaml);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(format!("{}", e).contains("Invalid line format"));
    }
}

/// Complex Combinations

#[test]
fn test_tokenize_complex_combination() {
    let yaml = r#"
company:
  name: "Tech Solutions Inc."
  location: "Silicon Valley"
  employees:
    - id: A001
      name: John Doe
      department: Engineering
      skills: ["Rust", "Python", "Docker"]
      is_manager: true
    - id: B002
      name: Jane Smith
      department: Marketing
      skills: ["SEO", "Content Creation"]
      is_manager: false
  projects:
    - name: Project Alpha
      status: In Progress
      budget: 150000.75
      team_members:
        - John Doe
        - Jane Smith
    - name: Project Beta
      status: Completed
      budget: 50000.00
      team_members:
        - Mike Johnson
  contact_info:
    email: info@techsolutions.com
    phone: "123-456-7890"
    website: "https://www.techsolutions.com"
"#; // raw string literal を使用

    let tokens = tokenize_string(yaml).unwrap();

    // 複雑な結合テストは、期待されるトークンリストを正確に構築する必要があります。
    // これは非常に長くなるため、一部を抜粋するか、構造を検証する形にします。
    // ここでは、一部のトークンシーケンスを検証します。
    assert!(!tokens.is_empty());
    assert_eq!(tokens[0], Token::Newline); // 先頭の空行
    assert_eq!(tokens[1], Token::Key("company :".to_string()));
    assert_eq!(tokens[2], Token::Newline);
    assert_eq!(tokens[3], Token::Indent(2));
    assert_eq!(
        tokens[4],
        Token::Key("name : \"Tech Solutions Inc.\"".to_string())
    );
    // ... 必要に応じてさらに多くのトークンを検証 ...

    // 最後のトークンがEofであることを確認
    assert_eq!(*tokens.last().unwrap(), Token::Eof);
}
