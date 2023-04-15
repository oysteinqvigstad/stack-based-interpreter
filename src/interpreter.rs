use crate::state::State;
use crate::token::{ProgramError, Token};



pub fn exec(state: &mut State) -> Result<Token, ProgramError> {
    exec_entry(state)?;

    match state.len() {
        0 => Err(ProgramError::StackEmpty),
        1 => Ok(state.peek()?.unwrap()),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}

pub fn exec_entry(state: &mut State) -> Result<(), ProgramError> {
    while !state.instruction_set.is_empty() {
        for item in state.instruction_set.pop_front() {
            match item.clone() {
                Token::Symbol(_) => match evaluate_operation(state, item)? {
                    Some(token) => state.push(token),
                    None => continue
                },
                Token::List(_) => {
                    match eval_list(state, item.clone())? {
                        Some(token) => state.push(token),
                        None => continue

                    }
                },
                _ => state.push(item)
            };
        }
    }
    Ok(())
}


fn evaluate_operation(stack: &mut State, token: Token) -> Result<Option<Token>, ProgramError> {
    match &token {
        Token::Symbol(s) => {
            match s.as_str() {
                "+" => exec_binary_op(stack, "+"),
                "-" => exec_binary_op(stack, "-"),
                "*" => exec_binary_op(stack, "*"),
                "/" => exec_binary_op(stack, "/"),
                "div" => exec_binary_op(stack, "div"),
                "<" => exec_binary_op(stack, "<"),
                ">" => exec_binary_op(stack, ">"),
                "==" => exec_binary_op(stack, "=="),
                "not" => exec_unary_op(stack, "not"),
                "&&" => exec_binary_op(stack, "&&"),
                "||" => exec_binary_op(stack, "||"),
                "length" => exec_unary_op(stack, "length"),
                "parseInteger" => exec_unary_op(stack, "parseInteger"),
                "parseFloat" => exec_unary_op(stack, "parseFloat"),
                "words" => exec_unary_op(stack, "words"),
                "pop" => exec_unary_op(stack, "pop"),
                "swap" => stack.stack_swap(),
                "dup" =>  stack.stack_dup(),
                "empty" => exec_unary_op(stack, "empty"),
                "head" => exec_unary_op(stack, "head"),
                "tail" => exec_unary_op(stack, "tail"),
                "cons" => exec_binary_op(stack, "cons"),
                "append" => exec_binary_op(stack, "append"),
                "exec" => exec_unary_op(stack, "exec"),
                "map" => exec_unary_op(stack, "map"),
                "each" => exec_unary_op(stack, "each"),
                "times" => exec_unary_op(stack, "times"),
                "foldl" => exec_binary_op(stack, "foldl"),
                "if" => exec_unary_op(stack, "if"),
                "loop" => stack.exec_loop(),
                ":=" => exec_binary_op(stack, ":="),
                x => stack.get_bind(x),
            }
        },
        _ => panic!("Non-operation cannot be executed")
    }
}

fn exec_binary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let right = stack.stack_pop()?;
    let left = stack.stack_pop()?;

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
        "cons" => right.cons(left),
        "append" => left.append(right),
        "foldl" => left.foldl(right, stack),
        ":=" => left.set_bind(right, stack),
        _ => Err(ProgramError::UnknownSymbol)
    }

}

fn exec_unary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let left = stack.stack_pop()?;
    match op {
        "pop" => Ok(None),
        "not" => left.not(),
        "length" => left.len(),
        "parseInteger" => left.parse_int(),
        "parseFloat" => left.parse_float(),
        "words" => left.words(),
        "empty" => left.empty(),
        "head" => left.head(),
        "tail" => left.tail(),
        "if" => left.if_exp(stack),
        "map" => left.map(stack),
        "exec" => left.exec(stack),
        "each" => left.each(stack),
        "times" => left.times(stack),
        _ => Err(ProgramError::UnknownSymbol)
    }
}

fn eval_list(state: &mut State, t: Token) -> Result<Option<Token>, ProgramError> {
    match t {
        Token::List(items) => {
            let mut updated_list = Vec::<Token>::new();
            for item in items {
                let token = match item {
                    Token::Symbol(op) => state.get_bind(op.as_str())?.unwrap(),
                    Token::List(_) => eval_list(state, item)?.unwrap(),
                    _ => item
                };
                updated_list.push(token);
            }
            Ok(Some(Token::List(updated_list)))
        },
        _ => Err(ProgramError::ExpectedList)
    }
}