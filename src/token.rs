use std::fmt;
use std::fmt::Formatter;
use std::iter::once;
use std::mem::discriminant;
use std::ops::{Add, Sub, Mul, Div};
use crate::token::ProgramError::ProgramFinishedWithMultipleValues;
// use crate::interpreter::exec;
use crate::parser::{lex};
use crate::state::State;


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
    Symbol(String),
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
            Token::Symbol(x) => write!(f, "{}", x),
        }
    }
}





impl Add for Token {
    type Output = Result<Option<Token>, ProgramError>;
    fn add(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x+y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x+y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}


impl Sub for Token {
    type Output = Result<Option<Token>, ProgramError>;
    fn sub(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x-y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x-y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

impl Mul for Token {
    type Output = Result<Option<Token>, ProgramError>;
    fn mul(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x*y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x*y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

// This is floating point division, even with ints, it will return a float
impl Div for Token {
    type Output = Result<Option<Token>, ProgramError>;
    fn div(self, other: Token) -> Self::Output {
        match type_coert(self, other)? {
            (_, Token::Int(0)) => Err(ProgramError::DivisionByZero),
            (_, Token::Float(0.0)) => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => rt(Token::Float(x as f32 /y as f32)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x/y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}
impl Token {
    pub fn int_div(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match type_coert(self, other)? {
            (_, Token::Int(0)) => Err(ProgramError::DivisionByZero),
            (_, Token::Float(0.0)) => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x / y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Int((x / y) as i128)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }

    pub fn lt(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Bool(x < y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Bool(x < y)),
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x < y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn gt(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Bool(x > y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Bool(x > y)),
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x > y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn eq(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match type_coert(self, other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Bool(x == y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Bool(x == y)),
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x == y)),
            (Token::String(x), Token::String(y)) => rt(Token::Bool(x == y)),
            (Token::List(x), Token::List(y)) => rt(Token::Bool(x == y)),
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn not(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::Bool(x) => rt(Token::Bool(!x)),
            _ => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn and(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x && y)),
            (_, _) => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn or(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x || y)),
            (_, _) => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn len(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::Int(x.len() as i128)),
            Token::Block(x) => rt(Token::Int(x.len() as i128)),
            Token::String(x) => rt(Token::Int(x.len() as i128)),
            _ => Err(ProgramError::ExpectedEnumerable)
        }
    }

    pub fn parse_int(self: Token) -> Result<Option<Token>, ProgramError> {
        println!("{:?}", self);
        match self {
            Token::String(x) => {
                match x.parse::<i128>() {
                    Ok(i) => rt(Token::Int(i)),
                    Err(_) => Err(ProgramError::NumberConversionError)
                }
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn parse_float(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::String(x) => {
                match x.parse::<f32>() {
                    Ok(f) => rt(Token::Float(f)),
                    Err(_) => Err(ProgramError::NumberConversionError)
                }
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn words(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::String(x) => {
                rt(Token::List(lex(x.as_str()).iter().map(|s| Token::String(s.to_string())).collect::<Vec<Token>>()))
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    pub fn empty(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::Bool(x.is_empty())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn head(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => {
                if !x.is_empty() { rt(x[0].clone()) } else { Err(ProgramError::ExpectedEnumerable) }
            }
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn tail(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::List(x.into_iter().skip(1).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn cons(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::List(x), a) => rt(Token::List(once(a).chain(x).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn append(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::List(x), Token::List(y)) => rt(Token::List(x.into_iter().chain(y.into_iter()).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }
}
/*
    pub fn exec(self: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match self {
            Token::Block(x) => {
                stack.stack.extend(x);
                crate::interpreter::exec(stack)
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }
    }


    pub fn if_exp(self, left: Token, right: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, left, right) {
            (Token::Bool(true), Token::Block(y), Token::Block(_)) => {
                stack.stack.extend(y);
                crate::interpreter::exec(stack)
            },
            (Token::Bool(false), Token::Block(_), Token::Block(z)) => {
                stack.stack.extend(z);
                crate::interpreter::exec(stack)
            },
            _ => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn map(self, other: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, other.clone()) {
            (Token::List(x), Token::Block(y)) => {
                let mut list: Vec<Token> = Vec::new();
                for item in x {
                    let result = exec(&mut State { stack: vec![item, other.clone(), Token::Symbol("exec".to_string())], bindings: stack.bindings.clone() })?;
                    list.push(result)
                }
                Ok(Token::List(list))
            },
            _ => Err(ProgramError::ExpectedList)

        }
    }

    pub fn each(self, other: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, other.clone()) {
            (Token::List(x), Token::Block(y)) => {
                for item in x {
                    let result = exec(&mut State { stack: vec![item, other.clone(), Token::Symbol("exec".to_string())], bindings: stack.bindings.clone() })?;
                    stack.push(result)
                }
                stack.pop()
            },
            (Token::List(x), _) => {
                println!("dette er {:?}", other);
                for item in x {
                    let result = exec(&mut State { stack: vec![item, other.clone()], bindings: stack.bindings.clone() })?;
                    stack.push(result)
                }
                stack.pop()
            }


            _ => Err(ProgramError::ExpectedList)
        }
    }

    pub fn times(self, other: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, other.clone()) {
            (Token::Int(x), Token::Block(y)) => {
                for i in 0..x {
                    stack.stack.extend(y.clone());
                }
                crate::interpreter::exec(stack)
            },
            (Token::Int(x), _) => {
                for i in 0..x {
                    stack.push(other.clone())
                }
                crate::interpreter::exec(stack)
            }
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }


    pub fn foldl(self, middle: Token, right: Token, stack: &mut State) -> Result<Token, ProgramError> {
        let mut sum = middle.clone();
        match (self, middle, right.clone()) {
            (Token::List(x), Token::Int(y), Token::Block(z)) => {
                for item in x {
                    sum = exec(&mut State { stack: vec![sum.clone(), item, right.clone(), Token::Symbol("exec".to_string())], bindings: stack.bindings.clone() })?;
                }
                Ok(sum)
            },
            (Token::List(x), Token::Int(y), Token::Symbol(z)) => {
                for item in x {
                    sum = exec(&mut State { stack: vec![sum.clone(), item, right.clone()], bindings: stack.bindings.clone() })?;
                }
                Ok(sum)
            },
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }

    pub fn while_loop(self, other: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, other) {
            (Token::Block(x), Token::Block(y)) => {
                loop {
                    // pushing condition to stack
                    stack.stack.extend(x.clone());
                    // checking condition
                    match crate::interpreter::exec(stack)? {
                        Token::Bool(true) => return stack.pop(),
                        Token::Bool(false) => {
                            stack.stack.extend(y.clone());

                            continue
                        }
                        _ => return Err(ProgramError::ExpectedBool)
                    }
                }
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }
    }

    pub fn bind(self, other: Token, stack: &mut State) -> Result<Token, ProgramError> {
        match (self, other.clone()) {
            (Token::Symbol(x), _) => {
                stack.bindings.insert(x, other.clone());
                exec(stack)
            },
            _ => Err(ProgramError::ExpectedVariable)
        }
    }




}

*/

fn rt<T>(value: T) -> Result<Option<T>, ProgramError> {
    Ok(Some(value))
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
