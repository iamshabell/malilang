use anyhow::Result;

use crate::{
    environment::Environment,
    expr::{ExpLiteralValue, Expr},
    lexer::Token,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
    Fun {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}

impl Stmt {
    pub fn execute(&self, env: &mut Environment) -> Result<()> {
        match self {
            Stmt::Fun { name, params, body } => {
                let function =
                    Function::new(name.clone(), params.clone(), body.clone(), env.clone());

                env.define(&name.lexeme, ExpLiteralValue::FunctionValue(function));
            }
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    enclosing: Environment,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>, enclosing: Environment) -> Self {
        Self {
            name,
            params,
            body,
            enclosing,
        }
    }

    pub fn call(&self, args: Vec<ExpLiteralValue>) -> Result<ExpLiteralValue> {
        let mut env = Environment::enclosing(self.enclosing.clone());

        for (param, arg) in self.params.iter().zip(args.iter()) {
            env.define(&param.lexeme, arg.clone());
        }

        for statement in &self.body {
            statement.execute(&mut env)?;
        }

        Ok(ExpLiteralValue::Nil)
    }
}
