use std::ops::Div;
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
        Err(_) => Err(ProgramError::StackEmpty),
        Ok(t) =>
            match t {
                Token::Float(_) => Ok(t),
                Token::Int(_) => Ok(t),
                Token::Bool(_) => Ok(t),
                Token::Operation(_) => evaluate_operation(stack, t),
                _ => Err(ProgramError::UnknownSymbol)
            }
    }
}

fn evaluate_operation(stack: &mut Stack, token: Token) -> Result<Token, ProgramError> {
    match &token {
        Token::Operation(s) => {
            let result = match s.as_str() {
                // arithmetic
                "+" => exec_binary_op(stack, "+")?,
                "-" => exec_binary_op(stack, "-")?,
                "*" => exec_binary_op(stack, "*")?,
                "/" => exec_binary_op(stack, "/")?,
                "div" => exec_binary_op(stack, "div")?,
                //logical
                "<" => exec_binary_op(stack, "<")?,
                ">" => exec_binary_op(stack, ">")?,
                "==" => exec_binary_op(stack, "==")?,
                "not" => exec_unary_op(stack, "not")?,
                "&&" => exec_binary_op(stack, "&&")?,
                "||" => exec_binary_op(stack, "||")?,
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
        "div" => left.int_div(right),
        "<" => left.lt(right),
        ">" => left.gt(right),
        "==" => left.eq(right),
        "&&" => left.and(right),
        "||" => left.or(right),
        _ => Err(ProgramError::UnknownSymbol)
    }
}

fn exec_unary_op(stack: &mut Stack, op: &str) -> Result<Token, ProgramError> {
    let left = exec(stack)?;
    match op {
        "not" => left.not(),
        _ => Err(ProgramError::UnknownSymbol)
    }
}

