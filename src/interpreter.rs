use crate::stack::Stack;
use crate::enums::{ProgramError, Token};



pub fn exec(stack: &mut Stack) -> Result<Token, ProgramError> {
    println!("{:?}", stack);
    match stack.pop() {
        Err(_) => Err(ProgramError::StackEmpty),
        Ok(t) =>
            match t {
                Token::Operation(_) => evaluate_operation(stack, t),
                _ => Ok(t)
            }
    }
}


fn evaluate_operation(stack: &mut Stack, token: Token) -> Result<Token, ProgramError> {
    match &token {
        Token::Operation(s) => {
            let result = match s.as_str() {
                "+" => exec_binary_op(stack, "+", true, true)?,
                "-" => exec_binary_op(stack, "-", true, true)?,
                "*" => exec_binary_op(stack, "*", true, true)?,
                "/" => exec_binary_op(stack, "/", true, true)?,
                "div" => exec_binary_op(stack, "div", true, true)?,
                "<" => exec_binary_op(stack, "<", true, true)?,
                ">" => exec_binary_op(stack, ">", true, true)?,
                "==" => exec_binary_op(stack, "==", true, true)?,
                "not" => exec_unary_op(stack, "not")?,
                "&&" => exec_binary_op(stack, "&&", true, true)?,
                "||" => exec_binary_op(stack, "||", true, true)?,
                "length" => exec_unary_op(stack, "length")?,
                "parseInteger" => exec_unary_op(stack, "parseInteger")?,
                "parseFloat" => exec_unary_op(stack, "parseFloat")?,
                "pop" => exec_binary_op(stack, "pop", true, true)?,
                "swap" => exec_binary_op(stack, "swap", true, true)?,
                "dup" => exec_unary_op(stack, "dup")?,
                "words" => exec_unary_op(stack, "words")?,
                "empty" => exec_unary_op(stack, "empty")?,
                "head" => exec_unary_op(stack, "head")?,
                "tail" => exec_unary_op(stack, "tail")?,
                "cons" => exec_binary_op(stack, "cons", true, true)?,
                "append" => exec_binary_op(stack, "append", true, true)?,
                "exec" => exec_unary_op(stack, "exec")?,
                "if" => exec_ternary_op(stack, "if", true, true, true)?,
                "map" => exec_binary_op(stack, "map", true, true)?,
                "each" => exec_binary_op(stack, "each", true, false)?,
                "times" => exec_binary_op(stack, "times", true, false)?,
                "foldl" => exec_ternary_op(stack, "foldl", true, false, false)?,
                _ => Err(ProgramError::UnknownSymbol)?
            };
            Ok(result)
        },
        _ => panic!("Non-operation cannot be executed")
    }
}


// this function that operations can be multiplied as well instead of evaluating whats beneath it
// `l` and `r` will allow operator recursion if the params are true
fn exec_binary_op(stack: &mut Stack, op: &str, l: bool, r: bool) -> Result<Token, ProgramError> {
    let right = match r {
        true => exec(stack)?,
        false => stack.pop()?,
    };
    let left = match l {
        true => exec(stack)?,
        false => stack.pop()?,
    };

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
        "map" => left.map(right, stack),
        "times" => left.times(right, stack),
        "each" => left.each(right, stack),
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


fn exec_ternary_op(stack: &mut Stack, op: &str, l: bool, m: bool, r: bool) -> Result<Token, ProgramError> {
    let right = match r {
        true => exec(stack)?,
        false => stack.pop()?,
    };
    println!("right: {}", right);
    let middle = match m {
        true => exec(stack)?,
        false => stack.pop()?,
    };
    println!("middle: {}", middle);
    let left = match l {
        true => exec(stack)?,
        false => stack.pop()?,
    };
    println!("left: {}", left);
    match op {
        "if" => left.if_exp(middle, right, stack),
        "foldl" => left.foldl(middle, right),
        _ => Err(ProgramError::UnknownSymbol)
    }
}
