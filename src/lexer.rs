use anyhow::Result;

pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            position: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.lex_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.position,
        ));

        Ok(self.tokens.clone())
    }

    fn lex_token(&mut self) -> Result<()> {
        let ch = self.advance();

        match ch {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.position += 1,
            '"' => self.string()?,
            cha => {
                if cha.is_digit(10) {
                    self.number();
                } else if cha.is_alphabetic() {
                    self.identifier();
                } else {
                    eprintln!("ERROR: Unexpected character: {}", cha);
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) -> Result<()> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.input[self.start..self.current];

        let token_type = match text {
            "qeyb" => TokenType::Class,
            "markasta" => TokenType::For,
            "kale" => TokenType::Else,
            "been" => TokenType::False,
            "hawl" => TokenType::Fun,
            "haddii" => TokenType::If,
            "waxba" => TokenType::Nil,
            "ama" => TokenType::Or,
            "daabac" => TokenType::Print,
            "celi" => TokenType::Return,
            "super" => TokenType::Super,
            "kan" => TokenType::This,
            "run" => TokenType::True,
            "weel" => TokenType::Var,
            "inta" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);

        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.input[self.start..self.current].parse::<f32>()?;
        self.add_token_literal(TokenType::Number, LiteralValue::FloatValue(value));

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.position += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("ERROR: Unterminated string");
            return Ok(());
        }

        self.advance();

        let value = self.input[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::StringLit, LiteralValue::StringValue(value));

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.input.chars().nth(self.current).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.input.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.input[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            None,
            self.position,
        ));
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: LiteralValue) {
        let text = &self.input[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            Some(literal),
            self.position,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        let c = self.input.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f32),
    StringValue(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    StringLit,
    Number,

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

    EOF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_tokens() {
        let source = "(){},.-+;*/";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Star,
            TokenType::Slash,
            TokenType::EOF,
        ];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn comparison_tokens_lex() {
        let source = "! != = == < <= > >=";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::Equal,
            TokenType::EqualEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::EOF,
        ];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn whitespace_tokens() {
        let source = " \t\n\r";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![TokenType::EOF];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn number_tokens() {
        let source = "123 123.45";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![TokenType::Number, TokenType::Number, TokenType::EOF];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn string_tokens() {
        let source = "\"hello world\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![TokenType::StringLit, TokenType::EOF];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn identifier_tokens() {
        let source = "foo bar baz";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::EOF,
        ];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn keyword_tokens() {
        let source =
            "qeyb markasta haddii kale been hawl waxba ama daabac celi super kan run weel inta";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![
            TokenType::Class,
            TokenType::For,
            TokenType::If,
            TokenType::Else,
            TokenType::False,
            TokenType::Fun,
            TokenType::Nil,
            TokenType::Or,
            TokenType::Print,
            TokenType::Return,
            TokenType::Super,
            TokenType::This,
            TokenType::True,
            TokenType::Var,
            TokenType::While,
            TokenType::EOF,
        ];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }

    #[test]
    fn handle_keywords() {
        let source = "weel foo = 123;\n qeyb bar = \"hello\";\ninta run { daabac bar; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();

        let expected_types = vec![
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::Class,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::StringLit,
            TokenType::Semicolon,
            TokenType::While,
            TokenType::True,
            TokenType::LeftBrace,
            TokenType::Print,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::EOF,
        ];

        let actual_types: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();

        assert_eq!(actual_types, expected_types);
    }
}
