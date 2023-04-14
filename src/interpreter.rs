use crate::state::State;
use crate::token::{ProgramError, Token};
use crate::token::ProgramError::UnknownSymbol;


// pub fn exec(stack: &mut State) -> Result<Token, ProgramError> {
//     println!("{:?}", stack);
//     match stack.pop() {
//         Err(_) => Err(ProgramError::StackEmpty),
//         Ok(t) =>
//             match t {
//                 Token::Symbol(_) => evaluate_operation(stack, t),
//                 _ => Ok(t)
//             }
//     }
// }

pub fn exec_entry(state: &mut State) -> Result<Token, ProgramError> {
    while !state.instruction_set.is_empty() {
        for item in state.instruction_set.pop_front() {
            match item.clone() {
                Token::Symbol(x) => match evaluate_operation(state, item)? {
                    Some(token) => state.push(token),
                    _ => continue
                }
                _ => state.push(item)
            };
        }
    }

    println!("After execution: {:?}", state);

    match state.len() {
        1 => state.pop(),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }

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
                "pop" => nullary_op(stack, "pop"),
                "swap" => nullary_op(stack, "swap"),
                "dup" => nullary_op(stack, "dup"),
                "empty" => exec_unary_op(stack, "empty"),
                "head" => exec_unary_op(stack, "head"),
                "tail" => exec_unary_op(stack, "tail"),
                "cons" => exec_binary_op(stack, "cons"),
                "append" => exec_binary_op(stack, "append"),
                // "exec" => exec_unary_op(stack, "exec")?,
                // "if" => exec_ternary_op(stack, "if", true, true, true)?,
                // "map" => exec_binary_op_old(stack, "map", true, true)?,
                // "each" => exec_binary_op_old(stack, "each", true, false)?,
                // "times" => exec_binary_op_old(stack, "times", true, false)?,
                // "foldl" => exec_ternary_op(stack, "foldl", true, false, false)?,
                // "loop" => exec_binary_op_old(stack, "loop", false, false)?,
                // ":=" => exec_binary_op_old(stack, ":=", true, true)?,
                // x => {
                //     match stack.bindings.get(x) {
                //         Some(t) => t.clone(),
                //         None => return Ok(Some(Token::Symbol(x.to_string())))
                _ => Err(ProgramError::UnknownSymbol)
            }
        },
        _ => panic!("Non-operation cannot be executed")
    }
}


fn exec_binary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let right = stack.pop()?;
    let left = stack.pop()?;

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

        // "map" => left.map(right, stack),
        // "times" => left.times(right, stack),
        // "each" => left.each(right, stack),
        // "loop" => left.while_loop(right, stack),
        // ":=" => left.bind(right, stack),
        _ => Err(ProgramError::UnknownSymbol)
    }

}




fn exec_unary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let left = stack.pop()?;
    match op {
        "not" => left.not(),
        "length" => left.len(),
        "parseInteger" => left.parse_int(),
        "parseFloat" => left.parse_float(),
        "dup" => Ok(Some(left)),
        "words" => left.words(),
        "empty" => left.empty(),
        "head" => left.head(),
        "tail" => left.tail(),
        // "exec" => left.exec(stack),
            _ => Err(ProgramError::UnknownSymbol)
    }
}


// fn exec_ternary_op(stack: &mut State, op: &str, l: bool, m: bool, r: bool) -> Result<Token, ProgramError> {
//     let right = match r {
//         true => exec(stack)?,
//         false => stack.pop()?,
//     };
//     println!("right: {}", right);
//     let middle = match m {
//         true => exec(stack)?,
//         false => stack.pop()?,
//     };
//     println!("middle: {}", middle);
//     let left = match l {
//         true => exec(stack)?,
//         false => stack.pop()?,
//     };
//     println!("left: {}", left);
//     match op {
        // "if" => left.if_exp(middle, right, stack),
        // "foldl" => left.foldl(middle, right, stack),
        // _ => Err(ProgramError::UnknownSymbol)
    // }
// }
fn nullary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    match op {
        "pop" => stack.pop()?,
        "swap" => stack.swap()?,
        _ => Err(ProgramError::UnknownSymbol)?
    };
    Ok(None)

}
