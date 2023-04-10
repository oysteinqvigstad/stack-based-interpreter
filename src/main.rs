pub mod parser;
mod stack;
use parser::parse;
use crate::tokenizer::tokenize;

mod interpreter;
mod tokenizer;

fn main() {
    tokenize("-5 + false 5.501");
}
