use anyhow::Result;

use crate::{
    expr::{ExpLiteralValue, Expr},
    lexer::{Token, TokenType},
};

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Primary,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.parse_expression(Precedence::None)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr> {
        let mut left = self.parse_prefix()?;

        if precedence < self.get_precedence() {
            let token = self.advance();
            left = self.parse_infix(left, token)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr> {
        let token = self.advance();

        match token.token_type {
            TokenType::Number
            | TokenType::StringLit
            | TokenType::False
            | TokenType::True
            | TokenType::Nil => Ok(Expr::Literal {
                value: ExpLiteralValue::from_token(token),
            }),
            TokenType::Identifier => Ok(Expr::Variable { name: token }),
            TokenType::Minus | TokenType::Bang => {
                let right = self.parse_expression(Precedence::Unary)?;
                Ok(Expr::Unary {
                    operator: token,
                    right: Box::new(right),
                })
            }
            TokenType::LeftParen => {
                let mut expr = self.parse_expression(Precedence::None)?;
                self.consume(TokenType::RightParen, "Exprected ')' after expression");
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => anyhow::bail!("Unexpected token: {:?}", token),
        }
    }

    fn parse_infix(&mut self, left: Expr, token: Token) -> Result<Expr> {
        let precedence = self.get_precedence();
        let right = self.parse_expression(precedence)?;

        Ok(Expr::Binary {
            left: Box::new(left),
            operator: token,
            right: Box::new(right),
        })
    }

    fn get_precedence(&self) -> Precedence {
        let token = self.peek();
        match token.token_type {
            TokenType::EqualEqual => Precedence::Equality,
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Precedence::Comparison,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            TokenType::Equal => Precedence::Assignment,
            TokenType::Bang => Precedence::Unary,
            _ => Precedence::None,
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(message.to_string())
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parser() {
        let input = "1 + 2 * 3";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap().to_string();

        assert_eq!(expr, "(+ 1 (* 2 3))");
    }

    #[test]
    fn test_parser_with_grouping() {
        let input = "(1 + 2) * 3";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap().to_string();

        assert_eq!(expr, "(* (+ 1 2) 3)");
    }

    #[test]
    fn test_parser_with_unary() {
        let input = "-1 + 2";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap().to_string();

        assert_eq!(expr, "(+ (- 1) 2)");
    }

    #[test]
    fn test_parser_with_grouping_and_unary() {
        let input = "(-1 + 2) * 3";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap().to_string();

        assert_eq!(expr, "(* (+ (- 1) 2) 3)");
    }
}
