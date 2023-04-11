#[derive(Debug)]
pub enum ProgramError {
    StackEmpty,
    UnknownSymbol,
    ExpectedBool,
    ExpectedBoolOrNumber,
    ExpectedEnumerable,
    ExpectedQuotation,
    ExpectedList,
    ExpectedVariable,
    DivisionByZero,
    ProgramFinishedWithMultipleValues,
    NumberConversionError,
}

#[derive(Debug)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation
}

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
