mod stack;

use crate::enums::Token;
use crate::interpreter::execute;
use crate::parser::parse;

mod interpreter;
mod parser;
mod enums;

fn main() {
    let mut res = parse("True True [ 5 3 2 ] ||");
    // println!("{}", Token::List(vec![Token::Bool(true), Token::Int(5)]));
    // println!("{:?}", parse("True [ False ] 5 25.2"));
    match res {
        Ok(x) => {
            let mut test = x;
            println!("{}", &test);
            println!("{:?}", execute(&mut test));
        },
        Err(x) => println!("{:?}", x)
    }

}
