use std::mem::discriminant;
use std::ops::{Add, Sub, Mul, Div};


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




impl Add for Token {
    type Output = Result<Token, ProgramError>;
    fn add(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x+y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Float(x+y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}


impl Sub for Token {
    type Output = Result<Token, ProgramError>;
    fn sub(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x-y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Float(x-y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

impl Mul for Token {
    type Output = Result<Token, ProgramError>;
    fn mul(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x*y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Float(x*y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

impl Div for Token {
    type Output = Result<Token, ProgramError>;
    fn div(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (_, Token::Int(0)) => Err(ProgramError::DivisionByZero),
            (_, Token::Float(0.0)) => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x-y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Float(x-y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}


fn type_coert(left: Token, right: Token) -> Result<(Token, Token), ProgramError> {
    if discriminant(&left) == discriminant(&right) {
        return Ok((left, right))
    }
    match (left, right) {
        (Token::Int(x), Token::Float(y)) => Ok((Token::Float(x as f32), Token::Float(y))),
        (Token::Float(x), Token::Int(y)) => Ok((Token::Float(x), Token::Float(y as f32))),
        _ => Err(ProgramError::NumberConversionError)
    }
}
