use crate::state::State;
use crate::token::{ProgramError, Token};
use crate::token::ProgramError::UnknownSymbol;



pub fn exec(state: &mut State) -> Result<Token, ProgramError> {
    exec_entry(state);
    println!("After execution: {:?}", state);

    match state.len() {
        0 => Err(ProgramError::StackEmpty),
        1 => state.stack_pop(),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}


pub fn exec_entry(state: &mut State) -> Result<(), ProgramError> {
    while !state.instruction_set.is_empty() {
        for item in state.instruction_set.pop_front() {
            match item.clone() {
                Token::Symbol(x) => match evaluate_operation(state, item)? {
                    Some(token) => state.push(token),
                    None => {
                        // println!("item: {:?} stack: {:?}", item.clone(), state.stack);

                        continue
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
        // "loop" => left.while_loop(right, stack),
        // ":=" => left.bind(right, stack),
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


fn exec_ternary_op(stack: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let right = stack.stack_pop()?;
    let middle = stack.stack_pop()?;
    let left = stack.stack_pop()?;
    match op {
        _ => Err(ProgramError::UnknownSymbol)
    }
}
