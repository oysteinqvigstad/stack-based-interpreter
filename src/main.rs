use prog2006_assignment_2::repl_mode;

fn main() {
    select_mode();
}

fn select_mode() {
    if let Err(e) = repl_mode() {
        println!("IO error: {}", e);
    }

}