use std::fmt;
use std::fmt::Formatter;
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

#[derive(Debug, Clone)]
pub enum Token {
    String(String),
    Int(i64),
    Float(f32),
    Bool(bool),
    List(Vec<Token>),
    Block(Vec<Token>),
    Operation(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::String(x) => write!(f, "\"{}\"", x),
            Token::Int(x) => write!(f, "{}", x),
            Token::Float(x) => write!(f, "{}", x),
            Token::Bool(x) => write!(f, "{}", if *x {"True"} else {"False"}),
            Token::List(x) => write!(f, "[{}]", x.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")),
            Token::Block(x) => write!(f, "{{{}}}", x.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ")),
            Token::Operation(x) => write!(f, "{}", x),
            _ => write!(f, "")
        }
    }
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
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x/y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Float(x/y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

impl Token {
    pub fn int_div(self, other: Token) -> Result<Token, ProgramError> {
        match type_coert(self, other)? {
            (_, Token::Int(0)) => Err(ProgramError::DivisionByZero),
            (_, Token::Float(0.0)) => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => Ok(Token::Int(x/y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Int((x/y) as i64)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }

    pub fn lt(self, other: Token) -> Result<Token, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Bool(x < y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Bool(x < y)),
            (Token::Bool(x), Token::Bool(y)) => Ok(Token::Bool(x < y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn gt(self, other: Token) -> Result<Token, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Bool(x > y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Bool(x > y)),
            (Token::Bool(x), Token::Bool(y)) => Ok(Token::Bool(x > y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn eq(self, other: Token) -> Result<Token, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => Ok(Token::Bool(x == y)),
            (Token::Float(x), Token::Float(y)) => Ok(Token::Bool(x == y)),
            (Token::Bool(x), Token::Bool(y)) => Ok(Token::Bool(x == y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn not(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::Bool(x) => Ok(Token::Bool(!x)),
            _ => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn and(self, other: Token) -> Result<Token, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => Ok(Token::Bool(x && y)),
            (_, _) => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn or(self, other: Token) -> Result<Token, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => Ok(Token::Bool(x || y)),
            (_, _) => Err(ProgramError::ExpectedBool)
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
