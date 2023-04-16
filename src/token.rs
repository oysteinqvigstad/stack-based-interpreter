use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::iter::once;
use std::mem::discriminant;
use std::ops::{Add, Sub, Mul, Div};
use crate::interpreter::exec;
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
    pub fn exec(self: Token, state: &mut State) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::Block(x) => {
                x.iter().rev().for_each(|t| state.instruction_set.push_front(t.clone()));
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }
    }


    pub fn if_exp(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let middle = state.pop_instruction()?;
        let right = state.pop_instruction()?;

        match (self, middle, right) {
            (Token::Bool(true), Token::Block(y), _) => {
                y.iter().rev().for_each(|t| state.instruction_set.push_front(t.clone()));
                Ok(None)
            },
            (Token::Bool(false), _, Token::Block(z)) => {
                z.iter().rev().for_each(|t| state.instruction_set.push_front(t.clone()));
                Ok(None)
            },
            (Token::Bool(true), y, _) => {
                state.instruction_set.push_front(y);
                Ok(None)
            },
            (Token::Bool(false), _, z) => {
                state.instruction_set.push_front(z);
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedBool)
        }
    }

    pub fn map(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.pop_instruction()?;
        match (self, right.clone()) {
            (Token::List(x), Token::Block(_)) => {
                let mut list: Vec<Token> = Vec::new();
                for item in x {
                    let instructions = vec![item, right.clone(), Token::Symbol("exec".to_string())];
                    let result = exec(&mut State { stack: vec![], instruction_set: VecDeque::from(instructions), bindings: state.bindings.clone(), functions: HashMap::<String, Token>::new() })?;
                    list.push(result)
                }
                rt(Token::List(list))
            },
            _ => Err(ProgramError::ExpectedList)

        }
    }

    pub fn each(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.pop_instruction()?;
        if let (Token::List(x), _) = (self, right.clone()) {
            for item in x {
                let mut instructions = vec![item, right.clone()];
                if let Token::Block(_) = right {
                    instructions.push(Token::Symbol("exec".to_string()));
                }
                let mut temp_state = State::from(state);
                temp_state.instruction_set = VecDeque::from(instructions);
                state.push(exec(&mut temp_state)?)
            }
            Ok(None)
        } else {
            Err(ProgramError::ExpectedList)
        }
    }

    pub fn times(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.pop_instruction()?;
        match (self, right.clone()) {
            (Token::Int(x), Token::Block(_)) => {
                for _ in 0..x {
                    right.clone().exec(state);
                }
                Ok(None)
            },
            (Token::Int(x), _) => {
                for _ in 0..x {
                    state.instruction_set.push_front(right.clone())
                }
                Ok(None)
            }
            _ => Err(ProgramError::ExpectedBoolOrNumber)
        }
    }


    pub fn foldl(self, middle: Token, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.pop_instruction()?;
        let mut sum = middle.clone();
        match (self, middle, &right) {
            (Token::List(x), Token::Int(_), Token::Block(_)) |
            (Token::List(x), Token::Int(_), Token::Symbol(_)) => {
                for item in x {
                    let mut temp_state = State::from(state);
                    temp_state.instruction_set = VecDeque::from(vec![sum.clone(), item, right.clone()]);
                    if let Token::Block(_) = right {
                        temp_state.instruction_set.push_back(Token::Symbol("exec".to_string()));
                    }
                    sum = exec(&mut temp_state)?;
                }
                rt(sum)
            },
            _ => Err(ProgramError::ExpectedList)
        }
    }


    pub fn set_bind(self, other: Token, stack: &mut State) -> Result<Option<Token>, ProgramError> {
        match (self, other.clone()) {
            (Token::Symbol(x), _) => {
                stack.bindings.insert(x, other.clone());
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedVariable)
        }
    }

    pub fn set_fun(self, other: Token, stack: &mut State) -> Result<Option<Token>, ProgramError> {
        match (self, other.clone()) {
            (Token::Symbol(x), Token::Block(_)) => {
                stack.functions.insert(x, other.clone());
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedVariable)
        }
    }

}


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
