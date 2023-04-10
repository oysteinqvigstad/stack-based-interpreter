pub mod parser;
mod stack;
use parser::parse;
use crate::tokenizer::tokenize;
use crate::tokenizer::lex;

mod interpreter;
mod tokenizer;

fn main() {
    let res = tokenize(lex("-5 + false { 5.501  [ 5 + 52 ] } 2.5 3").as_slice());
    println!("{:?}", res.tokens);
}
