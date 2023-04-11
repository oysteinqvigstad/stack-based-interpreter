use crate::enums::{ProgramError, Token};

#[derive(Debug)]
pub struct Stack {
    pub(crate) tokens: Vec<Token>
}

impl Stack {
    fn push(&mut self, token: Token) {
        self.tokens.push(token)
    }

    fn pop(&mut self) -> Result<Token, ProgramError> {
        match self.tokens.pop() {
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }
}