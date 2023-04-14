use prog2006_assignment_2::{parse, execute};

fn main() {
    let res = parse("10 20 swap");
    match res {
        Ok(x) => {
            let mut test = x;
            println!("Before running: {:?}", &test);
            println!("Result: {:?}", execute(&mut test));
        },
        Err(x) => println!("{:?}", x)
    }
}
