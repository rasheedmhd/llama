use crate::expr::Literal;
use std::fmt;

#[derive(Debug)]
pub struct Return {
    pub value: Literal,
}

impl Return {
    pub fn new(value: Literal) -> Self {
        Return { value }
    }
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Return exception with value: {:?}", self.value)
    }
}
