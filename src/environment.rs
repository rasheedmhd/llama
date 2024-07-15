use std::collections::HashMap;
use crate::expr::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;

pub struct Environment {
    pub values: HashMap<String, Literal>
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {

    pub fn new() -> Self { Self { values: HashMap::new() } }
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> EnvResult {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        };

        return Err(RuntimeError {
            token: name.clone(),
            msg: format!("OOpsie, looks like you forgot to define {} as a variable, (scratches head)", name.lexeme),
        });
    }

    pub fn assign(&mut self, name: Token, value: Literal ) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        };
        return Err(RuntimeError {
            token: name.clone(),
            msg: format!("OOpsie, looks like you forgot to define {} as a variable, (scratches head)", name.lexeme),
        });

    }
}

// private Expr assignment() {
// Expr expr = equality();
// if (match(EQUAL)) {
//     Token equals = previous();
//     Expr value = assignment();
//     if (expr instanceof Expr.Variable) {
//     T   oken name = ((Expr.Variable)expr).name;
//     r   eturn new Expr.Assign(name, value);
//     }
//     error(equals, "Invalid assignment target.");
// }
// return expr;
// }