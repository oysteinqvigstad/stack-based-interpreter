use std::ops::Div;
use crate::stack::Stack;
use crate::enums;
use crate::enums::{ProgramError, Token};



pub fn exec(stack: &mut Stack) -> Result<Token, ProgramError> {
    match stack.pop() {
        Err(_) => Err(ProgramError::StackEmpty),
        Ok(t) =>
            match t {
                Token::Float(_) => Ok(t),
                Token::Int(_) => Ok(t),
                Token::Bool(_) => Ok(t),
                Token::String(_) => Ok(t),
                Token::List(_) => Ok(t),
                Token::Block(_) => Ok(t),
                Token::Operation(_) => evaluate_operation(stack, t),
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
                "length" => exec_unary_op(stack, "length")?,
                "parseInteger" => exec_unary_op(stack, "parseInteger")?,
                "parseFloat" => exec_unary_op(stack, "parseFloat")?,
                "pop" => exec_binary_op(stack, "pop")?,
                "swap" => exec_binary_op(stack, "swap")?,
                "dup" => exec_unary_op(stack, "dup")?,
                "words" => exec_unary_op(stack, "words")?,
                "empty" => exec_unary_op(stack, "empty")?,
                "head" => exec_unary_op(stack, "head")?,
                "tail" => exec_unary_op(stack, "tail")?,
                "cons" => exec_binary_op(stack, "cons")?,
                "append" => exec_binary_op(stack, "append")?,
                "exec" => exec_unary_op(stack, "exec")?,
                "if" => exec_ternary_op(stack, "if")?,
                _ => Err(ProgramError::UnknownSymbol)?
            };
            Ok(result)
        },
        _ => panic!("Non-operation cannot be executed")
    }
}


fn exec_binary_op(stack: &mut Stack, op: &str) -> Result<Token, ProgramError> {
    let right = exec(stack)?;
    let left = exec(stack)?;
    if op == "swap" {
        stack.push(right.clone())
    }

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
        "pop" => Ok(left),
        "swap" => Ok(left),
        "cons" => right.cons(left),
        "append" => left.append(right),
        _ => Err(ProgramError::UnknownSymbol)
    }
}

fn exec_unary_op(stack: &mut Stack, op: &str) -> Result<Token, ProgramError> {
    let left = exec(stack)?;
    if op == "dup" {
        stack.push(left.clone())
    }
    match op {
        "not" => left.not(),
        "length" => left.len(),
        "parseInteger" => left.parse_int(),
        "parseFloat" => left.parse_float(),
        "dup" => Ok(left),
        "words" => left.words(),
        "empty" => left.empty(),
        "head" => left.head(),
        "tail" => left.tail(),
        "exec" => left.exec(stack),
            _ => Err(ProgramError::UnknownSymbol)
    }
}

fn exec_ternary_op(stack: &mut Stack, op: &str) -> Result<Token, ProgramError> {
    let right = exec(stack)?;
    let middle = exec(stack)?;
    let left = exec(stack)?;
    match op {
        "if" => left.if_exp(middle, right, stack),
        _ => Err(ProgramError::UnknownSymbol)
    }
}