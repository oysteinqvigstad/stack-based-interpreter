use std::cmp::Ordering;
use crate::state::State;
use std::collections::VecDeque;
use std::process::exit;
use crate::token::Token;
use crate::error::ProgramError;


/// Entry point for the interpreter
///
/// The interpreter borrows a mutable state and makes changes to it.
/// It also returns the token on the stack if there's only one left.
/// If there are zero or more, then it returns a warning instead
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete or not exactly one
/// item is left on the stack
///
pub fn execute_program(state: &mut State) -> Result<Token, ProgramError> {
    start_runtime(state)?;

    match state.len() {
        0 => Err(ProgramError::StackEmpty),
        1 => Ok(state.stack_peek()?.unwrap()),
        _ => Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}


/// Execution component of the interpreter
///
/// As long as there are instructions left, it will pop them off one by one
/// and determine what kind it is. If it is a operation, it will send it to
/// the dispatcher. If it is a list, then it will try to replace bound items and
/// then add to the stack. For all other items, they are immediately placed on the stack
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
pub fn start_runtime(state: &mut State) -> Result<(), ProgramError> {
    while !state.instruction_set.is_empty() {
        if let Some(item) = state.instruction_set.pop_front() {
            match item {
                Token::Symbol(op) => match dispatch_operation(state, op.as_str())? {
                    Some(token) => state.stack_push(token),
                    None => continue
                },
                Token::List(_) => {
                    match replace_items_with_bindings(state, item.clone())? {
                        Some(token) => state.stack_push(token),
                        None => continue
                    }
                },
                _ => state.stack_push(item)
            }
        }
    }
    Ok(())
}

/// Primary dispatcher for operations
///
/// It will forward the operation depending on how many arguments it takes. Some operations
/// interact entirely with the stack and takes no arguments.
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
/// * `op` - The operation in string form to be executed
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn dispatch_operation(state: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let unary_ops = ["not", "length", "parseInteger", "parseFloat", "print", "words", "pop",
                     "empty", "head", "tail", "exec", "map", "each", "times", "if", "print"];
    let binary_ops = ["+", "-", "*", "/", "<", ">", "==", "&&", "||", "div", "append", "cons",
                      "foldl", ":=", "fun"];

    if unary_ops.contains(&op) {
        dispatch_unary_operation(state, op)
    } else if binary_ops.contains(&op) {
        dispatch_binary_operation(state, op)
    } else {
        dispatch_nullary_operation(state, op)
    }
}

/// Secondary dispatcher for binary operations
///
/// This function takes two arguments from the stack before passing to the respective function
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
/// * `op` - The operation in string form to be executed
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn dispatch_binary_operation(state: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let right = state.stack_pop()?;
    let left = state.stack_pop()?;

    match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "div" => left.int_div(right),
        "<" => left.compare(right, Ordering::Less),
        ">" => left.compare(right, Ordering::Greater),
        "==" => left.compare(right, Ordering::Equal),
        "&&" => left.and(right),
        "||" => left.or(right),
        "cons" => right.cons(left),
        "append" => left.append(right),
        "foldl" => left.foldl(right, state),
        ":=" => left.set_bind(right, state),
        "fun" => left.set_fun(right, state),
        _ => Err(ProgramError::UnknownSymbol)
    }

}

/// Secondary dispatcher for unary operations
///
/// This function takes one argument from the stack before passing to the respective function
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
/// * `op` - The operation in string form to be executed
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn dispatch_unary_operation(state: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    let left = state.stack_pop()?;
    match op {
        "pop" => Ok(None),
        "not" => left.not(),
        "length" => left.len(),
        "parseInteger" => left.parse_int(),
        "parseFloat" => left.parse_float(),
        "print" => left.print(),
        "words" => left.words(),
        "empty" => left.empty(),
        "head" => left.head(),
        "tail" => left.tail(),
        "if" => left.if_exp(state),
        "map" => left.map(state),
        "exec" => left.exec(state),
        "each" => left.each(state),
        "times" => left.times(state),
        _ => Err(ProgramError::UnknownSymbol)
    }
}


/// Secondary dispatcher for nullary operations
///
/// This function takes zero arguments from the stack before passing to the respective function
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
/// * `op` - The operation in string form to be executed
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn dispatch_nullary_operation(state: &mut State, op: &str) -> Result<Option<Token>, ProgramError> {
    match op {
        "swap" => state.stack_swap(),
        "dup" =>  state.stack_dup(),
        "'" => state.stack_add_unbound(),
        "read" => state.read(),
        ":b" => state.display(op),
        ":f" => state.display(op),
        ":q" => exit(0),
        "loop" => execute_loop(state),
        x => state.resolve_symbol(x, true),
    }
}



/// Transforms a list by replacing any known bindings
///
/// If no known bindings are found, no transformation is made to the item
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
/// * `t` - The list token
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn replace_items_with_bindings(state: &mut State, t: Token) -> Result<Option<Token>, ProgramError> {
    match t {
        Token::List(items) => {
            let mut updated_list = Vec::<Token>::new();
            for item in items {
                let token = match item {
                    Token::Symbol(op) => state.resolve_symbol(op.as_str(), true)?.unwrap(),
                    Token::List(_) => replace_items_with_bindings(state, item)?.unwrap(),
                    _ => item
                };
                updated_list.push(token);
            }
            Ok(Some(Token::List(updated_list)))
        },
        _ => Err(ProgramError::ExpectedList)
    }
}

/// Execute Conditional loops for the program.
///
/// The condition is taken from the instruction list, not the stack
///
/// # Arguments
///
/// * `state` - The stack, instruction list, list of functions and bindings
///
/// # Errors
///
/// Returns ProgramError if the operation cannot complete
///
fn execute_loop(state: &mut State) -> Result<Option<Token>, ProgramError> {
    let break_condition = state.instruction_pop(true)?;
    let block = state.instruction_pop(true)?;
    let break_eval = vec![break_condition.clone(), Token::Symbol("exec".to_string())];
    let code_block = vec![block.clone(), Token::Symbol("exec".to_string())];

    match break_condition {
        Token::Block(_) => {
            let mut temp_state = state.clone();
            temp_state.instruction_set = VecDeque::from(break_eval.clone());

            loop {
                start_runtime(&mut temp_state)?;
                match temp_state.stack_pop()? {
                    Token::Bool(true) => {
                        // take the resulting stack and return
                        state.stack = temp_state.stack.clone();
                        return Ok(None)
                    },
                    Token::Bool(false) => {
                        // run the code block and then evaluate again
                        let mut both: Vec<Token> = vec![];
                        both.extend(code_block.clone());
                        both.extend(break_eval.clone());
                        temp_state.instruction_set = VecDeque::from(both);
                        continue
                    },
                    _ => return Err(ProgramError::ExpectedBool)
                }
            }
        },
        _ => Err(ProgramError::ExpectedQuotation)
    }

}
