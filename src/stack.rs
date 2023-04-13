use std::fmt;
use std::iter::Map;
use crate::enums::{ProgramError, Token};

#[derive(Debug)]
pub struct State {
    pub(crate) stack: Vec<Token>,
    pub(crate) bindings: Map<String, Vec<Token>>
}


impl fmt::Display for State {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.stack.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
   }
}




impl State {
    pub fn push(&mut self, token: Token) {
        self.stack.push(token)
    }

    pub fn pop(&mut self) -> Result<Token, ProgramError> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }

    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

}
