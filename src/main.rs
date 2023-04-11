mod stack;
use crate::parser::parse;

mod interpreter;
mod parser;
mod enums;

fn main() {
    let res = parse("-5 43 [ true -42. 42 3 \" Hello \" ] } ");
    match res {
        Ok(x) => println!("{:?}", x.tokens),
        Err(x) => println!("{:?}", x)
    }
}
