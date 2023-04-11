mod stack;

use crate::enums::Token;
use crate::interpreter::exec;
use crate::parser::parse;

mod interpreter;
mod parser;
mod enums;

fn main() {
    let mut res = parse("5 0 / 55 ");
    // println!("{:?}", Token::Float(10.0) / Token::Int(1));
    match res {
        Ok(x) => {
            let mut test = x;
            println!("{:?}", exec(&mut test));
        },
            // println!("{:?}", x.tokens);
            // println!("{:?}", exec(&mut x)),

        Err(x) => println!("{:?}", x)
    }

}
