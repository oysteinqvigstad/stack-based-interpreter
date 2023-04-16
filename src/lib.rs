mod parser;
mod interpreter;
mod token;
mod state;

use state::State;
use crate::interpreter::execute_program;
use std::io::{self, BufRead, Error, Write};
use crate::parser::parse_string_to_instructions;

/// Read-Evaluate-Print-Loop (REPL)
///
/// This is one of two modes the program can operate in. REPL will read input
/// from the user in a loop, continuously operating on the same stack and
/// report back any warnings provided by the program
///
/// # Errors
///
/// Returns IO error if reading from `stdout`, such as if non UTF-8 chars are encountered
///
pub fn repl_mode() -> Result<(), Error> {
    // create handle for input and output stream
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut state = State::new();

    loop {
        let mut line = String::new();
        print!("bprog > ");
        // flush output buffer to ensure that the prompt is immediately visible
        stdout.flush()?;
        // lock stdin, read a line, and store the user input
        stdin.lock().read_line(&mut line).expect("Could not read from stdin");
        // parse the input into tokens and store it in the instruction list
        match parse_string_to_instructions(line.as_str(), &mut state) {
            // if successful, execute the tokens and print the result
            Ok(_) => {
                // execute and print the interpreted results
                match execute_program(&mut state) {
                    Ok(_) => println!("stack : {}", state),
                    Err(e) => println!("stack : {}\nwarn  : {:?}", state, e)
                }
            },
            // if unsuccessful, clear the instructions and print the error
            Err(e) => {
                println!("stack : {}\nerror : {:?}", state, e);
                state.instruction_set.clear();
            }
        }
    }
}


/// Utility function used for integration testing
///
/// Any tests in `/tests/tests.rs` will pass through here. It takes a string,
/// parses it, executes it, and evaluates it against the test
///
/// # Arguments
///
/// * `input` - input string to be parsed and executed
///
pub fn t(input: &str) -> String {
    let mut state = State::new();
    match parse_string_to_instructions(input, &mut state) {
        Ok(_) => {
            match execute_program(&mut state) {
                Ok(r) => format!("{}", r),
                Err(e) => format!("{:?}", e)
            }},
        Err(e) => panic!("{:?}", e)
    }

}

