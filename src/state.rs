use std::fmt;
use std::collections::{HashMap, VecDeque};
use crate::interpreter::exec_entry;
use crate::token::{ProgramError, Token};

#[derive(Debug, Clone)]
pub struct State {
    pub(crate) stack: Vec<Token>,
    pub(crate) instruction_set: VecDeque<Token>,
    pub(crate) bindings: HashMap<String, Token>,
    pub(crate) functions: HashMap<String, Token>
}


impl fmt::Display for State {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.stack.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
   }
}



impl State {
    pub fn new() -> Self {
        let stack: Vec<Token> = Vec::new();
        let instruction_set: VecDeque<Token> = VecDeque::new();
        let bindings: HashMap<String, Token> = HashMap::new();
        let functions: HashMap<String, Token> = HashMap::new();
        Self { stack, instruction_set, bindings, functions }
    }

    pub fn from(other: &State) -> Self {
        let stack: Vec<Token> = Vec::new();
        let instruction_set: VecDeque<Token> = VecDeque::new();
        let bindings = other.bindings.clone();
        let functions = other.functions.clone();
        Self { stack, instruction_set, bindings, functions }
    }


    pub fn push(&mut self, token: Token) {
        self.stack.push(token)
    }

    pub fn stack_pop(&mut self) -> Result<Token, ProgramError> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }

    pub fn stack_swap(&mut self) -> Result<Option<Token>, ProgramError> {
        let right = self.stack_pop()?;
        let left = self.stack_pop()?;
        self.push(right);
        self.push(left);
        Ok(None)
        
    }

    pub fn stack_dup(&mut self) -> Result<Option<Token>, ProgramError> {
        let left = self.stack_pop()?;
        self.push(left.clone());
        self.push(left);
        Ok(None)
    }

    pub fn peek(&mut self) -> Result<Option<Token>, ProgramError> {
        let size = self.len();
        if size == 0 {
            Err(ProgramError::StackEmpty)
        } else {
            Ok(Some(self.stack[size - 1].clone()))
        }
    }

    pub fn get_instructions(self) -> Vec<Token> {
        self.instruction_set.into_iter().collect()

    }

    pub fn pop_instruction(&mut self) -> Result<Token, ProgramError> {
        match self.instruction_set.pop_front() {
            // Some(Token::Symbol(op)) => Ok(self.resolve_symbol(op.as_str())?.unwrap()),
            Some(Token::Symbol(op)) => {
                match self.resolve_symbol(op.as_str())? {
                    Some(binding) => Ok(binding),
                    None => Ok(Token::Symbol(op))
                }
                // Ok(self.resolve_symbol(op.as_str())?.unwrap())
            },
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }

    pub fn exec_loop(&mut self) -> Result<Option<Token>, ProgramError> {
        let break_condition = self.pop_instruction()?;
        let block = self.pop_instruction()?;
        let break_eval = vec![break_condition.clone(), Token::Symbol("exec".to_string())];
        let code_block = vec![block.clone(), Token::Symbol("exec".to_string())];

        match break_condition {
            Token::Block(_) => {
                let mut state = self.clone();
                state.instruction_set = VecDeque::from(break_eval.clone());

                loop {
                    exec_entry(&mut state)?;
                    match state.stack_pop()? {
                        Token::Bool(true) => {
                            // take the resulting stack and return
                            self.stack = state.stack.clone();
                            return Ok(None)
                        },
                        Token::Bool(false) => {
                            // run the code block and then evaluate again
                            let mut both: Vec<Token> = vec![];
                            both.extend(code_block.clone());
                            both.extend(break_eval.clone());
                            state.instruction_set = VecDeque::from(both);
                            continue
                        },
                        _ => return Err(ProgramError::ExpectedBool)
                    }
                }
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }

    }


    pub fn resolve_symbol(&mut self, op: &str) -> Result<Option<Token>, ProgramError> {
        // checking if there is a binding or a function. Function will take precedence
        if let Some(t) = self.functions.get(op) {
            self.instruction_set.push_front(Token::Symbol("exec".to_string()));
            self.instruction_set.push_front(t.clone());
            return Ok(None)
        }

        match self.bindings.get(op) {
            Some(t) => Ok(Some(t.clone())),
            None => Ok(Some(Token::Symbol(op.to_string())))
        }
    }



    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

}
