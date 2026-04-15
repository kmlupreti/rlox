pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<LiteralType>,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralType>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal: None,
        }
    }
}
impl ToString for Token {
    fn to_string(&self) -> String {
        // format!("{} {} {}", self.token_type, self.lexeme, self.literal)
        self.lexeme.clone()
    }
}
pub enum LiteralType {
    String,
    Int,
    Char,
    Callable,
}
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    Rightbrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
