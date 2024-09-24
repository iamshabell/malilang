use core::panic;

use crate::{
    environment::Environment,
    lexer::{LiteralValue, Token, TokenType},
};
use anyhow::Result;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ExpLiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

use ExpLiteralValue::*;

impl ExpLiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            ExpLiteralValue::Number(n) => n.to_string(),
            ExpLiteralValue::StringValue(s) => s.clone(),
            ExpLiteralValue::True => "true".to_string(),
            ExpLiteralValue::False => "false".to_string(),
            ExpLiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => match token.literal {
                Some(LiteralValue::IntValue(x)) => Self::Number(x as f32),
                Some(LiteralValue::FloatValue(x)) => Self::Number(x),
                _ => panic!("Could not unwrap as f32"),
            },
            TokenType::StringLit => match token.literal {
                Some(LiteralValue::StringValue(s)) => Self::StringValue(s),
                Some(LiteralValue::Identifier(s)) => Self::StringValue(s),
                _ => panic!("Could not unwrap as string"),
            },
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }

    pub fn from_bool(b: bool) -> ExpLiteralValue {
        if b {
            True
        } else {
            False
        }
    }

    pub fn is_falsy(&self) -> ExpLiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 {
                    True
                } else {
                    False
                }
            }
            StringValue(s) => {
                if s.is_empty() {
                    True
                } else {
                    False
                }
            }
            True => False,
            False => True,
            Nil => True,
        }
    }
}

pub enum Expr {
    Literal {
        value: ExpLiteralValue,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Assignment { name, value } => format!("({:?} = {:?})", name, value.to_string()),
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping { expression } => format!("{}", (*expression).to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpLiteralValue> {
        match self {
            Expr::Variable { name } => match name.lexeme.as_str() {
                "True" => Ok(ExpLiteralValue::True),
                "False" => Ok(ExpLiteralValue::False),
                "Nil" => Ok(ExpLiteralValue::Nil),
                _ => {
                    let result = env
                        .get(&name.lexeme)
                        .map(|v| v.clone())
                        .ok_or_else(|| anyhow::anyhow!("Undefined variable {:?}", name.lexeme));

                    result
                }
            },
            Expr::Assignment { name, value } => {
                let value = value.evaluate(env)?;
                env.assign(name.lexeme.as_str(), value.clone());
                Ok(value)
            }
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => expression.evaluate(env),
            Expr::Unary { operator, right } => {
                let right_value = right.evaluate(env)?;
                match (&right_value, operator.token_type) {
                    (ExpLiteralValue::Number(x), TokenType::Minus) => {
                        Ok(ExpLiteralValue::Number(-x))
                    }
                    (_, TokenType::Minus) => {
                        anyhow::bail!("Cannot negate {:?}", right_value);
                    }
                    (any, TokenType::Bang) => {
                        Ok(ExpLiteralValue::from_bool(any.is_falsy() == True))
                    }
                    (_, token_type) => {
                        anyhow::bail!(
                            "Cannot evaluate unary expression with operator {:?}",
                            token_type,
                        );
                    }
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = left.evaluate(env)?;
                let right_value = right.evaluate(env)?;

                match operator.token_type {
                    TokenType::Plus => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::Number(l + r))
                        }
                        (ExpLiteralValue::StringValue(l), ExpLiteralValue::StringValue(r)) => {
                            Ok(ExpLiteralValue::StringValue(format!("{}{}", l, r)))
                        }
                        (l, r) => anyhow::bail!("Cannot add {:?} and {:?}", l, r),
                    },
                    TokenType::Minus => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::Number(l - r))
                        }
                        (l, r) => anyhow::bail!("Cannot subtract {:?} and {:?}", l, r),
                    },
                    TokenType::Star => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::Number(l * r))
                        }
                        (l, r) => anyhow::bail!("Cannot multiply {:?} and {:?}", l, r),
                    },
                    TokenType::Slash => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            if r == 0.0 {
                                anyhow::bail!("Cannot divide by zero");
                            }
                            Ok(ExpLiteralValue::Number(l / r))
                        }
                        (l, r) => anyhow::bail!("Cannot divide {:?} and {:?}", l, r),
                    },
                    TokenType::Greater => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::from_bool(l > r))
                        }
                        (l, r) => anyhow::bail!("Cannot compare {:?} and {:?}", l, r),
                    },
                    TokenType::GreaterEqual => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::from_bool(l >= r))
                        }
                        (l, r) => anyhow::bail!("Cannot compare {:?} and {:?}", l, r),
                    },
                    TokenType::Less => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::from_bool(l < r))
                        }
                        (l, r) => anyhow::bail!("Cannot compare {:?} and {:?}", l, r),
                    },
                    TokenType::LessEqual => match (left_value, right_value) {
                        (ExpLiteralValue::Number(l), ExpLiteralValue::Number(r)) => {
                            Ok(ExpLiteralValue::from_bool(l <= r))
                        }
                        (l, r) => anyhow::bail!("Cannot compare {:?} and {:?}", l, r),
                    },
                    TokenType::EqualEqual => {
                        Ok(ExpLiteralValue::from_bool(left_value == right_value))
                    }
                    TokenType::BangEqual => {
                        Ok(ExpLiteralValue::from_bool(left_value != right_value))
                    }
                    _ => anyhow::bail!(
                        "Cannot evaluate binary expression with operator {:?}",
                        operator
                    ),
                }
            }

            _ => todo!("Implement evaluation for {:?}", self.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_evaluation() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(1.0),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(2.0),
            }),
        };

        let mut env = Environment::new();
        let result = expr.evaluate(&mut env).unwrap();

        assert_eq!(result, ExpLiteralValue::Number(3.0));
    }

    #[test]
    fn test_evaluation_unary() {
        let expr = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(1.0),
            }),
        };

        let mut env = Environment::new();
        let result = expr.evaluate(&mut env).unwrap();

        assert_eq!(result, ExpLiteralValue::Number(-1.0));
    }

    #[test]
    fn test_evaluation_comparison() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(1.0),
            }),
            operator: Token {
                token_type: TokenType::Greater,
                lexeme: ">".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(2.0),
            }),
        };

        let mut env = Environment::new();
        let result = expr.evaluate(&mut env).unwrap();

        assert_eq!(result, ExpLiteralValue::False);
    }

    #[test]
    fn test_multiple_grouping_operations() {
        // more like 1 + (2 * 3)
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: ExpLiteralValue::Number(1.0),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: ExpLiteralValue::Number(2.0),
                    }),
                    operator: Token {
                        token_type: TokenType::Star,
                        lexeme: "*".to_string(),
                        literal: None,
                        line_number: 0,
                    },
                    right: Box::new(Expr::Literal {
                        value: ExpLiteralValue::Number(3.0),
                    }),
                }),
            }),
        };

        let mut env = Environment::new();
        let result = expr.evaluate(&mut env).unwrap();

        assert_eq!(result, ExpLiteralValue::Number(7.0));
    }

    #[test]
    fn test_assignment() {}
}
