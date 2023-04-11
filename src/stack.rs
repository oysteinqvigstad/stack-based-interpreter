use std::fmt;
use crate::enums::{ProgramError, Token};

#[derive(Debug)]
pub struct Stack {
    pub(crate) tokens: Vec<Token>
}


impl fmt::Display for Stack {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.tokens.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
   }
}




impl Stack {
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token)
    }

    pub fn pop(&mut self) -> Result<Token, ProgramError> {
        match self.tokens.pop() {
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }

    pub fn len(&mut self) -> usize {
        self.tokens.len()
    }

}
