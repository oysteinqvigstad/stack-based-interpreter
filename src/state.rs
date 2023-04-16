use std::fmt;
use std::collections::{HashMap, VecDeque};
use crate::token::Token;
use crate::interpreter::ProgramError;



/// State holds the current state of the parsed/executed program
///
/// In REPL mode the state is considered global. However it may make
/// secondary temporary states for calculating mapped values, etc.
///
#[derive(Debug, Clone)]
pub struct State {
    pub(crate) stack: Vec<Token>,
    pub(crate) instruction_set: VecDeque<Token>,
    pub(crate) bindings: HashMap<String, Token>,
    pub(crate) functions: HashMap<String, Token>
}


impl fmt::Display for State {
    /// Formatter for printing the stack of the state
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stack.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
    }
}

impl State {
    /// Creates a new `State` instance with empty stack, instruction set, bindings, and functions.
    ///
    /// # Returns
    ///
    /// A new `State` instance.
    ///
    pub fn new() -> Self {
        let stack: Vec<Token> = Vec::new();
        let instruction_set: VecDeque<Token> = VecDeque::new();
        let bindings: HashMap<String, Token> = HashMap::new();
        let functions: HashMap<String, Token> = HashMap::new();
        Self { stack, instruction_set, bindings, functions }
    }

    /// Creates a new `State` instance based on an existing `State`,
    /// copying its bindings and functions.
    ///
    /// # Arguments
    ///
    /// * `other` - The existing `State` to copy bindings and functions from.
    ///
    /// # Returns
    ///
    /// A new `State` instance with the same bindings and functions as `other`.
    ///
    pub fn from(other: &State) -> Self {
        let stack: Vec<Token> = Vec::new();
        let instruction_set: VecDeque<Token> = VecDeque::new();
        let bindings = other.bindings.clone();
        let functions = other.functions.clone();
        Self { stack, instruction_set, bindings, functions }
    }

    /// Returns the current length of the stack.
    ///
    /// # Returns
    ///
    /// The number of elements in the stack.
    ///
    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

    /// Pushes a `Token` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `token` - The `Token` to push onto the stack.
    ///
    pub fn push(&mut self, token: Token) {
        self.stack.push(token)
    }

    pub fn stack_pop(&mut self) -> Result<Token, ProgramError> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(ProgramError::StackEmpty)
        }
    }

    /// Pops a `Token` from the stack.
    ///
    /// # Returns
    ///
    /// The popped `Token` if the stack is not empty, or a `ProgramError` if the stack is empty.
    ///
    pub fn stack_swap(&mut self) -> Result<Option<Token>, ProgramError> {
        let right = self.stack_pop()?;
        let left = self.stack_pop()?;
        self.push(right);
        self.push(left);
        Ok(None)
        
    }

    /// Duplicates the top element on the stack.
    ///
    /// # Returns
    ///
    /// `Ok(None)` if the duplication is successful, or a `ProgramError` if the stack is empty.
    ///
    pub fn stack_dup(&mut self) -> Result<Option<Token>, ProgramError> {
        let left = self.stack_pop()?;
        self.push(left.clone());
        self.push(left);
        Ok(None)
    }

    /// Returns the top element of the stack without removing it.
    ///
    /// # Returns
    ///
    /// `Ok(Some(Token))` containing the top element of the stack if the stack is not empty,
    /// or a `ProgramError::StackEmpty` error if the stack is empty.
    ///
    pub fn peek(&mut self) -> Result<Option<Token>, ProgramError> {
        let size = self.len();
        if size == 0 {
            Err(ProgramError::StackEmpty)
        } else {
            Ok(Some(self.stack[size - 1].clone()))
        }
    }

    /// Converts the instruction set stored in a `State` instance into a `Vec<Token>`.
    ///
    /// # Returns
    ///
    /// A `Vec<Token>` containing the instructions from the `State`'s instruction set.
    ///
    pub fn get_instructions(self) -> Vec<Token> {
        self.instruction_set.into_iter().collect()

    }

    /// Pops and returns the next instruction from the front of the instruction set.
    ///
    /// If the instruction is a symbol, it resolves the symbol to its corresponding
    /// binding or function.
    ///
    /// # Returns
    ///
    /// The next `Token` instruction if the instruction set is not empty,
    /// or a `ProgramError::StackEmpty` error if the instruction set is empty.
    ///
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

    /// Resolves a given symbol to its corresponding binding or function.
    ///
    /// Functions take precedence over bindings.
    ///
    /// # Arguments
    ///
    /// * `op` - The symbol to resolve.
    ///
    /// # Returns
    ///
    /// `Ok(Some(Token))` containing the resolved token if the symbol exists in the bindings
    /// or functions, or the original symbol as a `Token::Symbol` if not found.
    ///
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
}
