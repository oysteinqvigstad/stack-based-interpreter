use std::fmt;
use std::hash::Hash;
use std::collections::{HashMap, VecDeque};
use crate::token::{ProgramError, Token};

#[derive(Debug)]
pub struct State {
    pub(crate) stack: Vec<Token>,
    pub(crate) instruction_set: VecDeque<Token>,
    pub(crate) bindings: HashMap<String, Token>
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

    pub fn swap(&mut self) -> Result<Token, ProgramError> {
        let right = self.pop()?;
        let left = self.pop()?;
        self.push(right);
        Ok(left)
        
    }
    
    
    pub fn get_instructions(self) -> Vec<Token> {
        self.instruction_set.into_iter().collect()

    }



    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

}
