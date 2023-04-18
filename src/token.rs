use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::iter::once;
use std::mem::discriminant;
use std::ops::{Add, Sub, Mul, Div};
use crate::interpreter::execute_program;
use crate::parser::{lex};
use crate::state::State;
use crate::error::ProgramError;


/// Represents a single token in the language.
///
/// Each variant of the enum corresponds to a different type of token,
/// such as string, integer, float, boolean, list, block, or symbol.
///
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    String(String),
    Int(i128),
    Float(f32),
    Bool(bool),
    List(Vec<Token>),
    Block(Vec<Token>),
    Symbol(String),
}


/// Implements the `Display` trait for the `Token` enum.
///
/// This allows tokens to be converted into a human-readable string representation.
impl fmt::Display for Token {
    /// Formats the `Token` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
/// Implements the `Add` trait for the `Token` enum.
///
/// This enables addition operations between two tokens.
impl Add for Token {
    type Output = Result<Option<Token>, ProgramError>;

    /// Adds `self` to `other` and returns the result.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to add to `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token as the result of the addition or an error.
    fn add(self, other: Token) -> Self::Output {
        match self.type_coercion(other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x+y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x+y)),
            _ => Err(ProgramError::ExpectedNumber)
        }
    }
}

/// Implements the `Sub` trait for the `Token` enum.
///
/// This enables subtraction operations between two tokens.
impl Sub for Token {
    type Output = Result<Option<Token>, ProgramError>;

    /// Subtracts `other` from `self` and returns the result.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to subtract from `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token as the result of the subtraction or an error.
    ///
    fn sub(self, other: Token) -> Self::Output {
        match self.type_coercion(other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x-y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x-y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}
/// Implements the `Mul` trait for the `Token` enum.
///
/// This enables multiplication operations between two tokens.
impl Mul for Token {
    type Output = Result<Option<Token>, ProgramError>;

    /// Multiplies `self` by `other` and returns the result.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to multiply with `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token as the result of the multiplication or an error.
    ///
    fn mul(self, other: Token) -> Self::Output {
        match self.type_coercion(other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x*y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x*y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

/// Implements the `Div` trait for the `Token` enum.
///
/// This enables division operations between two tokens.
impl Div for Token {
    type Output = Result<Option<Token>, ProgramError>;

    /// Divides `self` by `other` and returns the result.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to divide `self` by.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token as the result of the division or an error.
    ///
    fn div(self, other: Token) -> Self::Output {
        match self.type_coercion(other)? {
            (_, Token::Int(y)) if y == 0 => Err(ProgramError::DivisionByZero),
            (_, Token::Float(y)) if y == 0.0 => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => rt(Token::Float(x as f32 /y as f32)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Float(x/y)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }
}

impl Token {

    /// Performs integer division between `self` and `other` and returns the result.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to divide `self` by.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token as the result of the integer division or an error.
    ///
    pub fn int_div(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match self.type_coercion(other)? {
            (_, Token::Int(y)) if y == 0 => Err(ProgramError::DivisionByZero),
            (_, Token::Float(y)) if y == 0.0 => Err(ProgramError::DivisionByZero),
            (Token::Int(x), Token::Int(y)) => rt(Token::Int(x / y)),
            (Token::Float(x), Token::Float(y)) => rt(Token::Int((x / y) as i128)),
            _ => Err(ProgramError::NumberConversionError)
        }
    }

    /// Compares `self` and `other` based on the specified comparison ordering.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to compare with `self`.
    /// * `comparison` - The desired comparison ordering (`Ordering::Less`,
    ///                  `Ordering::Greater`, or `Ordering::Equal`).
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the result of the comparison or an error.
    ///
    pub fn compare(self, other: Token, comparison: Ordering) -> Result<Option<Token>, ProgramError> {
        match self.type_coercion(other)? {
            (Token::Int(x), Token::Int(y)) => rt(Token::Bool(compare_values(&x, &y, comparison))),
            (Token::Float(x), Token::Float(y)) => rt(Token::Bool(compare_values(&x, &y, comparison))),
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(compare_values(&x, &y, comparison))),
            (Token::String(x), Token::String(y)) => rt(Token::Bool(compare_values(&x, &y, comparison))),
            (Token::List(x), Token::List(y)) => rt(Token::Bool(compare_values(&x, &y, comparison))),
            _ => Err(ProgramError::ExpectedBoolOrNumber),
        }
    }

    /// Returns the negation of the boolean value of `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the negation or an error.
    ///
    pub fn not(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::Bool(x) => rt(Token::Bool(!x)),
            _ => Err(ProgramError::ExpectedBool)
        }
    }

    /// Performs a logical AND operation between `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to perform the logical AND with `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the result of
    /// the AND operation or an error.
    ///
    pub fn and(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x && y)),
            (_, _) => Err(ProgramError::ExpectedBool)
        }
    }

    /// Performs a logical OR operation between `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to perform the logical OR with `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the result of
    /// the OR operation or an error.
    ///
    pub fn or(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::Bool(x), Token::Bool(y)) => rt(Token::Bool(x || y)),
            (_, _) => Err(ProgramError::ExpectedBool)
        }
    }

    /// Returns the length of `self` if it is a list, block, or string.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the length or an error.
    ///
    pub fn len(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::Int(x.len() as i128)),
            Token::Block(x) => rt(Token::Int(x.len() as i128)),
            Token::String(x) => rt(Token::Int(x.len() as i128)),
            _ => Err(ProgramError::ExpectedEnumerable)
        }
    }

    /// Parses the string value of `self` as an integer and returns the result.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the parsed integer or an error.
    ///
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

    /// Parses the string value of `self` as a floating-point number and returns the result.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the parsed floating-point number or an error.
    ///
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

    /// Splits the string value of `self` into words and returns a list of tokens.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the list of words or an error.
    ///
    pub fn words(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::String(x) => {
                rt(Token::List(lex(x.as_str()).iter().map(|s| Token::String(s.to_string())).collect::<Vec<Token>>()))
            },
            _ => Err(ProgramError::ExpectedString)
        }
    }

    /// Checks if the list value of `self` is empty and returns the result.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the emptiness check or an error.
    ///
    pub fn empty(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::Bool(x.is_empty())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    /// Returns the first element of the list value of `self`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the head of the list or an error.
    ///
    pub fn head(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => {
                if !x.is_empty() { rt(x[0].clone()) } else { Err(ProgramError::ExpectedEnumerable) }
            }
            _ => Err(ProgramError::ExpectedList)
        }
    }

    /// Returns a new list containing all elements of the list value of `self` except the first one.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the tail of the list or an error.
    ///
    pub fn tail(self: Token) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::List(x) => rt(Token::List(x.into_iter().skip(1).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    /// Adds a new token to the beginning of the list value of `self`.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to be added to the beginning of the list.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the new list or an error.
    ///
    pub fn cons(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::List(x), a) => rt(Token::List(once(a).chain(x).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }


    /// Appends the elements of the second list to the first list value of `self`.
    ///
    /// # Arguments
    ///
    /// * `other` - The token representing the list to be appended.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the resulting list or an error.
    ///
    pub fn append(self, other: Token) -> Result<Option<Token>, ProgramError> {
        match (self, other) {
            (Token::List(x), Token::List(y)) => rt(Token::List(x.into_iter().chain(y.into_iter()).collect())),
            _ => Err(ProgramError::ExpectedList)
        }
    }

    /// Executes a block of tokens by pushing them into the instruction set of the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - The mutable reference to the state where the block will be executed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn exec(self: Token, state: &mut State) -> Result<Option<Token>, ProgramError> {
        match self {
            Token::Block(x) => {
                x.iter().rev().for_each(|t| state.instruction_set.push_front(t.clone()));
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedQuotation)
        }
    }

    /// Executes one of the two given branches based on the boolean value of `self`.
    ///
    /// # Arguments
    ///
    /// * `state` - The mutable reference to the state where the conditional branches will be executed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn if_exp(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let middle = state.instruction_pop(false)?;
        let right = state.instruction_pop(false)?;

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

    /// Applies a function represented by a block of tokens to each element of the list value of `self`.
    ///
    /// # Arguments
    ///
    /// * `state` - The mutable reference to the state where the block will be executed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the transformed list or an error.
    ///
    pub fn map(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.instruction_pop(false)?;
        match (self, right.clone()) {
            (Token::List(x), Token::Block(_)) => {
                let mut list: Vec<Token> = Vec::new();
                for item in x {
                    let instructions = vec![item, right.clone(), Token::Symbol("exec".to_string())];
                    let result = execute_program(&mut State { stack: vec![], instruction_set: VecDeque::from(instructions), bindings: state.bindings.clone(), functions: HashMap::<String, Token>::new() })?;
                    list.push(result)
                }
                rt(Token::List(list))
            },
            _ => Err(ProgramError::ExpectedList)

        }
    }

    /// Applies a function or token to each element of the list value of `self`.
    ///
    /// # Arguments
    ///
    /// * `state` - The mutable reference to the state where the function will be executed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn each(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.instruction_pop(false)?;
        if let (Token::List(x), _) = (self, right.clone()) {
            for item in x {
                let mut instructions = vec![item, right.clone()];
                if let Token::Block(_) = right {
                    instructions.push(Token::Symbol("exec".to_string()));
                }
                let mut temp_state = State::from(state);
                temp_state.instruction_set = VecDeque::from(instructions);
                state.stack_push(execute_program(&mut temp_state)?)
            }
            Ok(None)
        } else {
            Err(ProgramError::ExpectedList)
        }
    }

    /// Repeats the execution of a block of tokens or pushes a token onto the state stack
    /// a specified number of times.
    ///
    /// # Arguments
    ///
    /// * `state` - The mutable reference to the state where the block will be executed
    ///             or the token will be pushed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn times(self, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.instruction_pop(false)?;
        match (self, right.clone()) {
            (Token::Int(x), Token::Block(_)) => {
                for _ in 0..x {
                    right.clone().exec(state)?;
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

    /// Applies a left-fold operation on a list of tokens with an initial accumulator value.
    ///
    /// # Arguments
    ///
    /// * `middle` - The initial accumulator value.
    /// * `state` - The mutable reference to the state where the function will be executed.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token representing the final accumulator value or an error.
    ///
    pub fn foldl(self, middle: Token, state: &mut State) -> Result<Option<Token>, ProgramError> {
        let right = state.instruction_pop(false)?;
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
                    sum = execute_program(&mut temp_state)?;
                }
                rt(sum)
            },
            _ => Err(ProgramError::ExpectedList)
        }
    }

    /// Binds a token to a symbol in the current state.
    ///
    /// # Arguments
    ///
    /// * `other` - The token to be bound.
    /// * `stack` - The mutable reference to the state where the binding will be stored.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn set_bind(self, other: Token, stack: &mut State) -> Result<Option<Token>, ProgramError> {
        match (self, other.clone()) {
            (Token::Symbol(x), _) => {
                stack.bindings.insert(x, other.clone());
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedVariable)
        }
    }

    /// Binds a block of tokens to a symbol as a function in the current state.
    ///
    /// # Arguments
    ///
    /// * `other` - The token representing the block of tokens to be bound as a function.
    /// * `stack` - The mutable reference to the state where the function binding will be stored.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional token or an error.
    ///
    pub fn set_fun(self, other: Token, stack: &mut State) -> Result<Option<Token>, ProgramError> {
        match (self, other.clone()) {
            (Token::Symbol(x), Token::Block(_)) => {
                stack.functions.insert(x, other.clone());
                Ok(None)
            },
            _ => Err(ProgramError::ExpectedVariable)
        }
    }


    /// Prints a token to stdout
    ///
    /// # Returns
    ///
    /// None
    ///
    pub fn print(self) -> Result<Option<Token>, ProgramError> {
        println!("{}", self);
        Ok(None)
    }


    /// Coerces the types of the two input tokens to a common type.
    ///
    /// # Arguments
    ///
    /// * `self` - The first token.
    /// * `right` - The second token.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple of the coerced tokens or an error.
    ///
    fn type_coercion(self, right: Token) -> Result<(Token, Token), ProgramError> {
        if discriminant(&self) == discriminant(&right) {
            return Ok((self, right))
        }
        match (self, right) {
            (Token::Int(x), Token::Float(y)) => Ok((Token::Float(x as f32), Token::Float(y))),
            (Token::Float(x), Token::Int(y)) => Ok((Token::Float(x), Token::Float(y as f32))),
            _ => Err(ProgramError::NumberConversionError)
        }
    }

}

/// Helper function that wraps the token in a `Result` containing an `Option`
///
/// # Arguments
///
/// * `token` - The value to be wrapped.
///
/// # Returns
///
/// A `Result` containing an `Option` with the token
///
fn rt<T>(token: T) -> Result<Option<T>, ProgramError> {
    Ok(Some(token))
}

/// Helper function that compares two values using a specified comparison operation. The
/// function is type templated such that it can be used in the compare method
///
/// # Arguments
///
/// * `x` - The first value.
/// * `y` - The second value.
/// * `comparison` - The comparison operation to be used (`Ordering::Less`,
///                  `Ordering::Greater`, or `Ordering::Equal`).
///
/// # Returns
///
/// A `bool` indicating the result of the comparison.
///
fn compare_values<T: PartialOrd + PartialEq>(x: &T, y: &T, comparison: Ordering) -> bool {
    match comparison {
        Ordering::Less => x < y,
        Ordering::Greater => x > y,
        Ordering::Equal => x == y,
    }
}