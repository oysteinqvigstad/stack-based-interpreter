use std::env;
use prog2006_assignment_2::{normal_mode, repl_mode};

/// The `main` function of the program. It checks for command line arguments to determine
/// the mode of operation. If the command line argument contains "repl", it starts the program
/// in REPL mode by calling `repl_mode`. If "repl" is not provided as an argument, the program
/// defaults to normal mode by calling `normal_mode`.
///
/// # Examples
///
/// To run the program in normal mode:
/// ```
/// $ cargo run < filename.txt
/// ```
///
/// To run the program in REPL mode:
/// ```
/// $ cargo run -- repl
/// ```
fn main() {
    let command_line_args: Vec<String> = env::args().collect();
    if command_line_args.contains(&"repl".to_string()) {
        repl_mode();
    } else {
        normal_mode();
    }
}
