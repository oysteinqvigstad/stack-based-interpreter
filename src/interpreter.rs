use crate::stack::Stack;
use crate::enums;
use crate::enums::{ProgramError, Token};

pub fn execute(stack: &mut Stack) -> Result<Token, ProgramError> {
    let result = exec(stack)?;
    match stack.len() {
        0 => Err(ProgramError::StackEmpty),
        1 => Ok(result),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}


pub fn exec(stack: &mut Stack) -> Result<Token, ProgramError> {
    match stack.pop() {
        Err(_) => panic!("Test"),
        Ok(t) =>
            match t {
                Token::Float(_) => Ok(t),
                Token::Int(_) => Ok(t),
                Token::Operation(_) => evaluate_operation(stack, t),
                _ => panic!("test")
            }
    }
}

fn evaluate_operation(stack: &mut Stack, token: Token) -> Result<Token, ProgramError> {
    match &token {
        Token::Operation(s) => {
            let result = match s.as_str() {
                "+" => exec_binary_op(stack, "+")?,
                "-" => exec_binary_op(stack, "-")?,
                "*" => exec_binary_op(stack, "*")?,
                "/" => exec_binary_op(stack, "/")?,
                _ => Err(ProgramError::UnknownSymbol)?
            };
            if stack.len() == 0 {
                stack.push(result.clone());
            }
            Ok(result)
        },
        _ => panic!("Non-operation cannot be executed")
    }
}


fn exec_binary_op(stack: &mut Stack, op: &str) -> Result<Token, ProgramError> {
    let right = exec(stack)?;
    let left = exec(stack)?;
    match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => Err(ProgramError::UnknownSymbol)
    }
}

fn op_arithmetic(op: &str) -> Result<Token, ProgramError> {

   Err(ProgramError::UnknownSymbol)
}

fn type_coert(left: Token, right: Token) -> Result<(Token, Token), ProgramError> {
    match (left, right) {
        (Token::Int(x), Token::Float(y)) => Ok((Token::Float(x as f32), Token::Float(y))),
        (Token::Float(x), Token::Int(y)) => Ok((Token::Float(x), Token::Float(y as f32))),
        _ => Err(ProgramError::NumberConversionError)
    }
}
