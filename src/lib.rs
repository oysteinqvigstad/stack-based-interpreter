mod parser;
mod interpreter;
mod token;
mod state;
mod error;

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
pub fn repl_mode() {
    let mut state = State::new();

    loop {
        // parse the input into tokens and store it in the instruction list
        match parse_string_to_instructions(read_input("bprog").as_str(), &mut state) {
            // if successful, execute the tokens and print the result
            Ok(_) => {
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

/// Reads a line of input from the user and returns it as a `String`.
///
/// This function prints a prompt to the user, reads a line of input from the user,
/// trims the newline character at the end, and returns the input as a `String`.
///
/// # Arguments
///
/// * `prompt` - A string slice that represents the prompt to be displayed before reading input.
///
/// # Returns
///
/// A `String` containing the user's input with the newline character removed.
///
pub fn read_input(prompt: &str) -> String {
    // create handle for input and output stream
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();
    print!("{} > ", prompt);
    // flush output buffer to ensure that the prompt is immediately visible
    let _ = stdout.flush();
    // read a line from user input
    stdin.lock().read_line(&mut line).expect("Could not read from stdin");
    line.trim_end_matches('\n').to_string()
}