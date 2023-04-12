use std::fmt;
use std::fmt::Formatter;
use std::io::Read;
use std::iter::once;
use std::mem::discriminant;
use std::ops::{Add, Sub, Mul, Div};
use crate::execute;
use crate::parser::{lex};
use crate::stack::Stack;


#[derive(Debug)]
pub enum ProgramError {
    StackEmpty,
    UnknownSymbol,
    ExpectedBool,
    ExpectedBoolOrNumber,
    ExpectedEnumerable,
    ExpectedQuotation,
    ExpectedString,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    String(String),
    Int(i128),
    Float(f32),
    Bool(bool),
    List(Vec<Token>),
    Block(Vec<Token>),
    Operation(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::String(x) => write!(f, "\"{}\"", x),
            Token::Int(x) => write!(f, "{}", x),
            Token::Float(x) => write!(f, "{:?}", x),
            Token::Bool(x) => write!(f, "{}", if *x {"True"} else {"False"}),
            Token::List(x) => write!(f, "[{}]", x.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")),
            Token::Block(x) => write!(f, "{{ {} }}", x.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ")),
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

// This is floating point division, even with ints, it will return a float
impl Div for Token {
    type Output = Result<Token, ProgramError>;
    fn div(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (_, Token::Int(0)) => Err(ProgramError::DivisionByZero),
            (_, Token::Float(0.0)) => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => Ok(Token::Float(x as f32 /y as f32)),
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
            (Token::Float(x), Token::Float(y)) => Ok(Token::Int((x/y) as i128)),
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
            (Token::String(x), Token::String(y)) => Ok(Token::Bool(x == y)),
            (Token::List(x), Token::List(y)) => Ok(Token::Bool(x == y)),
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

    pub fn len(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::List(x) => Ok(Token::Int(x.len() as i128)),
            Token::Block(x) => Ok(Token::Int(x.len() as i128)),
            Token::String(x) => Ok(Token::Int(x.len() as i128)),
            _ => Err(ProgramError::ExpectedEnumerable)
        }
    }

    pub fn parse_int(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::String(x) => {
                match x.parse::<i128>() {
                    Ok(i) => Ok(Token::Int(i)),
                    Err(_) => Err(ProgramError::NumberConversionError)
                }
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn parse_float(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::String(x) => {
                match x.parse::<f32>() {
                    Ok(f) => Ok(Token::Float(f)),
                    Err(_) => Err(ProgramError::NumberConversionError)
                }
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn words(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::String(x) => {
                Ok(Token::List(lex(x.as_str()).iter().map(|s| Token::String(s.to_string())).collect::<Vec<Token>>()))
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn empty(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::List(x) => Ok(Token::Bool(x.is_empty())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn head(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::List(x) => {
                if !x.is_empty() { Ok(x[0].clone()) } else { Err(ProgramError::ExpectedEnumerable) }
            }
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn tail(self: Token) -> Result<Token, ProgramError> {
        match self {
            Token::List(x) => Ok(Token::List(x.into_iter().skip(1).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn cons(self, other: Token) -> Result<Token, ProgramError> {
        match (self, other) {
            (Token::List(x), a) => Ok(Token::List(once(a).chain(x).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn append(self, other: Token) -> Result<Token, ProgramError> {
        match (self, other) {
            (Token::List(x), Token::List(y)) => Ok(Token::List(x.into_iter().chain(y.into_iter()).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn exec(self: Token, stack: &mut Stack) -> Result<Token, ProgramError> {
        match self {
            Token::Block(x) => {
                stack.tokens.extend(x);
                crate::interpreter::exec(stack)
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }
    }


    pub fn if_exp(self, left: Token, right: Token, stack: &mut Stack) -> Result<Token, ProgramError> {
        match (self, left, right) {
            (Token::Bool(true), Token::Block(y), Token::Block(z)) => {
                stack.tokens.extend(y);
                crate::interpreter::exec(stack)
            },
            (Token::Bool(false), Token::Block(y), Token::Block(z)) => {
                stack.tokens.extend(z);
                crate::interpreter::exec(stack)
            },
            _ => Err(ProgramError::ExpectedBool)
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
