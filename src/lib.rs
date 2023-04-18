mod parser;
mod interpreter;
mod token;
mod state;
mod error;

use state::State;
use crate::interpreter::execute_program;
use std::io::{self, BufRead, Write};
use std::process::exit;
use crate::parser::parse_string_to_instructions;

/// `repl_mode` starts a Read-Eval-Print Loop (REPL) that reads input lines, parses them as
/// instructions, and executes the instructions using a `State` object. After each execution,
/// it prints the current stack state. If an error occurs during execution, it prints the stack
/// state and a warning message. If there's an error in parsing the input string to instructions,
/// it prints the stack state, an error message, and clears the instruction set.
///
pub fn repl_mode() {
    let mut state = State::new();

    loop {
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

/// `normal_mode` reads input lines from standard input, parses them as instructions,
/// and executes the instructions using a `State` object. If the execution is successful,
/// it prints the result. If an error occurs during execution, it prints the error message.
/// If there's an error in parsing the input string to instructions, it prints the error
/// and exits the program with a status code of 1.
///
///
pub fn normal_mode() {
    let stdin = io::stdin();
    let mut state = State::new();
    for line in stdin.lock().lines() {
        if let Err(e) = parse_string_to_instructions(line.unwrap().as_str(), &mut state) {
            println!("{:?}", e);
            exit(1);
        }
    }
    match execute_program(&mut state) {
        Ok(token) => println!("{}", token),
        Err(e) => println!("{:?}", e)
    }
}
