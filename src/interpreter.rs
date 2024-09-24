use anyhow::Result;

use crate::{
    environment::Environment,
    expr::{ExpLiteralValue, Expr},
    statement::Stmt,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            stmt.execute(&mut self.environment)?;
        }

        Ok(())
    }
}
