
#[derive(Debug)]
pub enum Token {
    String(String),
    Int(i64),
    Float(f32),
    Bool(bool),
    List(Vec<Token>),
    Block(Vec<Token>),
    Operation(String)
}

#[derive(Debug)]
pub struct Stack {
    pub(crate) tokens: Vec<Token>
}

// impl Stack {
//
// }