/// Error types that may propagate during interpretation
#[derive(Debug)]
pub enum ProgramError {
    InstructionListEmpty,
    StackEmpty,
    UnknownSymbol,
    ExpectedBool,
    ExpectedBoolOrNumber,
    ExpectedNumber,
    ExpectedEnumerable,
    ExpectedQuotation,
    ExpectedString,
    ExpectedList,
    ExpectedVariable,
    ExpectedSymbol,
    DivisionByZero,
    ProgramFinishedWithMultipleValues,
    NumberConversionError,
}

/// Error types that may propagate during parsing
#[derive(Debug)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation
}
