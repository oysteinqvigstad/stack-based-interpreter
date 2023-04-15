mod parser;
mod interpreter;
mod token;
mod state;

use state::State;
use parser::{lex, tokenize_and_parse};
// use interpreter::exec;
use token::{ParserError, ProgramError, Token};
use crate::interpreter::exec;


pub fn parse(s: &str) -> Result<State, ParserError> {
    tokenize_and_parse(&lex(s))
}


pub fn execute(stack: &mut State) -> Result<Token, ProgramError> {
    let result = exec(stack)?;
    println!("DEBUG result after execute: {:?}", result);
    println!("DEBUG stack after execute: {:?}", stack);
    match stack.len() {
        0 => Ok(result),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}



pub fn t(input: &str) -> String {
    match parse(input) {
        Ok(a) => {
            let mut stack = a;
            match execute(&mut stack) {
                Ok(b) => format!("{b}"),
                Err(b) => format!("{:?}", b)
            }},
        Err(_) => panic!("n")
    }

}

