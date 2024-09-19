use crate::lexer::{LiteralValue, Token, TokenType};

pub enum ExpLiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

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
}
