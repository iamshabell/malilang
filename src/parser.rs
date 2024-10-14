use anyhow::Result;

use crate::{
    expr::{ExpLiteralValue, Expr},
    lexer::{Token, TokenType},
    statement::Stmt,
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        let peeked = self.peek().token_type;
        println!("Peeked: {:?}", peeked);

        let peeked_next = self.peek_next().token_type;
        if peeked_next == TokenType::LeftParen {
            self.advance();
            return self.parse_expression_statement();
        }

        match peeked {
            TokenType::Var => self.parse_variable_declaration(),
            TokenType::Print => self.parse_print_statement(),
            TokenType::Fun => self.parse_function_declaration(),
            TokenType::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt> {
        let expression = self.parse_expression(Precedence::None)?;
        println!("Expression: {:?}", expression);

        match self.consume(TokenType::Semicolon, "Expected ';' after expression") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Stmt::Expression { expression })
    }

    fn parse_block_statement(&mut self) -> Result<Stmt> {
        println!("Parsing block statement");
        match self.consume(TokenType::LeftBrace, "Expected '{' to start block") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        match self.consume(TokenType::RightBrace, "Expected '}' to end block") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Stmt::Block { statements })
    }

    fn parse_function_declaration(&mut self) -> Result<Stmt> {
        match self.consume(TokenType::Fun, "Expected 'hawl' keyword") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        let name = match self.consume(TokenType::Identifier, "Expected function name") {
            Ok(token) => token,
            Err(e) => anyhow::bail!(e),
        };

        match self.consume(TokenType::LeftParen, "Expected '(' after function name") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                match self.consume(TokenType::Identifier, "Expected parameter name") {
                    Ok(token) => params.push(token),
                    Err(e) => anyhow::bail!(e),
                }

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        match self.consume(TokenType::RightParen, "Expected ')' after parameters") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        match self.consume(TokenType::LeftBrace, "Expected '{' before function body") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }

        match self.consume(TokenType::RightBrace, "Expected '}' after function body") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Stmt::Fun { name, params, body })
    }

    fn parse_variable_declaration(&mut self) -> Result<Stmt> {
        match self.consume(TokenType::Var, "Expected 'weel' keyword") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }
        let name = match self.consume(TokenType::Identifier, "Expected variable name") {
            Ok(token) => token,
            Err(e) => anyhow::bail!(e),
        };

        let initializer = if self.match_token(TokenType::Equal) {
            Some(self.parse_expression(Precedence::None)?)
        } else {
            None
        };

        match self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        ) {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Stmt::Var {
            name,
            initializer: match initializer {
                Some(expr) => Some(expr),
                None => None,
            },
        })
    }

    fn parse_print_statement(&mut self) -> Result<Stmt> {
        match self.consume(TokenType::Print, "Expected 'print' keyword") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }
        let expression = self.parse_expression(Precedence::None)?;

        match self.consume(TokenType::Semicolon, "Expected ';' after print statement") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Stmt::Print { expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr> {
        let mut left = self.parse_prefix()?;

        if precedence < self.get_precedence() {
            let token = self.advance();
            println!("Token: {:?}", token.token_type);

            if token.token_type == TokenType::LeftParen {
                println!("Call expression");
                return self.finish_call(left);
            }
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
                let expr = self.parse_expression(Precedence::None)?;
                match self.consume(TokenType::RightParen, "Exprected ')' after expression") {
                    Ok(_) => (),
                    Err(e) => anyhow::bail!(e),
                }
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
        match token.token_type {
            TokenType::LeftParen => self.finish_call(left),
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Ok(Expr::Binary {
                left: Box::new(left),
                operator: token,
                right: Box::new(right),
            }),
            _ => anyhow::bail!("Unexpected token: {:?}", token),
        }
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.parse_expression(Precedence::None)?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        match self.consume(TokenType::RightParen, "Expected ')' after arguments") {
            Ok(_) => (),
            Err(e) => anyhow::bail!(e),
        }

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: self.previous(),
            arguments,
        })
    }

    fn get_precedence(&self) -> Precedence {
        let token = self.peek();
        match token.token_type {
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::Equality,
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Precedence::Comparison,
            TokenType::Plus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            TokenType::Equal => Precedence::Assignment,
            TokenType::Bang => Precedence::Unary,
            TokenType::Minus => {
                if self.peek().token_type == TokenType::Number {
                    Precedence::Term
                } else {
                    Precedence::Unary
                }
            }

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

    fn peek_next(&self) -> &Token {
        &self.tokens[self.current + 1]
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
        let expr = parser
            .parse_expression(Precedence::None)
            .unwrap()
            .to_string();

        assert_eq!(expr, "(+ 1 (* 2 3))");
    }

    #[test]
    fn test_parser_with_grouping() {
        let input = "(1 + 2) * 3";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser
            .parse_expression(Precedence::None)
            .unwrap()
            .to_string();

        assert_eq!(expr, "(* (+ 1 2) 3)");
    }

    #[test]
    fn test_parser_with_unary() {
        let input = "-1 + 2";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser
            .parse_expression(Precedence::None)
            .unwrap()
            .to_string();

        assert_eq!(expr, "(+ (- 1) 2)");
    }

    #[test]
    fn test_parser_with_grouping_and_unary() {
        let input = "(-1 + 2) * 3";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser
            .parse_expression(Precedence::None)
            .unwrap()
            .to_string();

        assert_eq!(expr, "(* (+ (- 1) 2) 3)");
    }
}
