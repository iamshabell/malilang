use anyhow::Result;

use crate::{
    environment::Environment,
    expr::{ExpLiteralValue, Expr},
    lexer::Token,
};

pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}

impl Stmt {
    pub fn execute(&self, env: &mut Environment) -> Result<()> {
        match self {
            Stmt::Expression { expression } => {
                expression.evaluate(env)?;
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate(env)?;
                println!("{}", value.to_string());
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => expr.evaluate(env)?,
                    None => ExpLiteralValue::Nil,
                };
                env.define(&name.lexeme, value);
            }
            Stmt::Block { statements } => {
                let mut block_env = Environment::enclosing(env.clone());

                for statement in statements {
                    statement.execute(&mut block_env)?;
                }
            }
        }

        Ok(())
    }
}
