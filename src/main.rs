use prog2006_assignment_2::repl_mode;

fn main() {
    select_mode();
}


/// Entry menu for the program
///
/// Lets user choose between REPL mode or read from file
///
fn select_mode() {
    if let Err(e) = repl_mode() {
        println!("IO error: {}", e);
    }

}