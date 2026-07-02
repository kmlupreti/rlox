use lox::{scanner::Scanner, token::Token, token_type::TokenType};

#[test]
fn should_skip_comments() {
    let source = "// hello this is test
(
// another test
*
";
    let mut scanner = Scanner::new(&String::from(source));
    let mut tokens = scanner.scan_tokens().unwrap().into_iter();
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::LeftParen, String::from("("), 2)
    );
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::Star, String::from("*"), 4)
    );
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::Eof, String::from("\0"), 5)
    )
}

#[test]
fn should_recognize_keywords() {
    let keyword_types = vec![
        TokenType::And,
        TokenType::Class,
        TokenType::Else,
        TokenType::False,
        TokenType::Fun,
        TokenType::For,
        TokenType::If,
        TokenType::Nil,
        TokenType::Or,
        TokenType::Print,
        TokenType::Return,
        TokenType::Super,
        TokenType::This,
        TokenType::True,
        TokenType::Var,
        TokenType::While,
        TokenType::Eof,
    ];
    let source =
        "and class else false fun for  if nil or print return  super this true var while eof";
    let source_chars = source.split_whitespace();
    let mut scanner = Scanner::new(&String::from(source));
    let mut tokens = scanner.scan_tokens().unwrap().into_iter();
    for (i, c) in source_chars.enumerate() {
        assert_eq!(
            tokens.next().unwrap(),
            &Token::new(keyword_types[i], String::from(c), 1)
        );
    }
}
#[test]
fn should_recognize_single_char_tokens() {
    let source = "(){}.;,+-*";
    let source_chars = source.chars();
    let mut scanner = Scanner::new(&String::from(source));
    let mut tokens = scanner.scan_tokens().unwrap().into_iter();
    for c in source_chars {
        assert_eq!(
            tokens.next().unwrap(),
            &Token::new(single_char_to_token_type(c), String::from(c), 1)
        );
    }
}

fn single_char_to_token_type(c: char) -> TokenType {
    match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::Rightbrace,
        '.' => TokenType::Dot,
        ';' => TokenType::Semicolon,
        ',' => TokenType::Comma,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Star,
        '\0' => TokenType::Eof,
        _ => TokenType::Nil,
    }
}
