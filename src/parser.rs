use std::collections::{HashMap, VecDeque};
use std::sync::atomic::spin_loop_hint;
use crate::token::Token;
use crate::token::ParserError;
use crate::state::State;




// tokenize breaks down a string into a Stack of tokens
pub fn tokenize_and_parse(words: &[&str]) -> Result<State, ParserError> {
    let mut stack: Vec<Token> = Vec::new();
    let mut instruction_set: VecDeque<Token> = VecDeque::new();
    let bindings: HashMap<String, Token> = HashMap::new();
    let mut index: usize = 0;

    while index < words.len() {
        let token = get_token(&mut index, words, &mut instruction_set)?;
        instruction_set.push_back(token);
        index += 1;
    }

    Ok(State { stack: stack, instruction_set: instruction_set, bindings: bindings })
}

pub fn get_token(index: &mut usize, words: &[&str], stack: &mut VecDeque<Token>) -> Result<Token, ParserError> {
    match words[index.clone()] {
        "[" => make_collection(index, words, Token::List(vec![])),
        "{" => make_collection(index, words, Token::Block(vec![])),
        "\"" => make_string(index, words),
        "]" => Err(ParserError::IncompleteList),
        "}" => Err(ParserError::IncompleteQuotation),
        "if" => make_if_expression(index, words, stack),
        "map" => make_binary_infix_to_postfix(index, words, stack),
        "each" => make_binary_infix_to_postfix(index, words, stack),
        "times" => make_binary_infix_to_postfix(index, words, stack),
        "foldl" => make_binary_infix_to_postfix(index, words, stack),
        "loop" => make_binary_prefix_to_postfix(index, words, stack),
        // ":=" => make_binary_infix_to_postfix(index, words, stack),
        s if is_bool(s) => Ok(Token::Bool(s.to_lowercase().parse::<bool>().unwrap())),
        s if is_integer(s) => Ok(Token::Int(s.parse::<i128>().unwrap())),
        s if is_float(s) => Ok(Token::Float(s.parse::<f32>().unwrap())),
        s => Ok(Token::Symbol(s.to_string()))
    }
}






pub fn lex(input: &str) -> Box<[&str]> {
    let vec: Vec<&str> = input.split_whitespace().collect();
    vec.into_boxed_slice()
}

// is_integer checks if the token is an integer. Also checks for negative numbers
fn is_integer(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-'));
    return at_least_one_digit && rest_are_legal
}

// is_integer checks if the token is a float. Also checks for negative numbers
fn is_float(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let exactly_one_dot = s.chars().filter(|c| *c == '.').count() == 1;
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-') || c == '.');
    at_least_one_digit && exactly_one_dot && rest_are_legal
}

// is_bool checks if the token is a bool
fn is_bool(s: &str) -> bool {
    s == "true" || s == "True" || s == "false" || s == "False"
}


fn make_collection(index: &mut usize, words: &[&str], t: Token) -> Result<Token, ParserError> {
    let mut level = 0;
    let start_index = *index + 1;
    while *index < words.len() {
        level += match (&t, words[*index]) {
            (Token::List(_),  "[") =>  1,
            (Token::List(_),  "]") => -1,
            (Token::Block(_), "{") =>  1,
            (Token::Block(_), "}") => -1,
            _                      =>  0,
        };

        if level == 0 {
            break;
        } else {
            *index += 1;
        }
    }
    match (t, level) {
        (Token::List(_), 0)  => Ok(Token::List(tokenize_and_parse(&words[start_index..*index])?.get_instructions())),
        (Token::Block(_), 0) => Ok(Token::Block(tokenize_and_parse(&words[start_index..*index])?.get_instructions())),
        (Token::List(_), _)  => Err(ParserError::IncompleteList),
        (Token::Block(_), _) => Err(ParserError::IncompleteQuotation),
        _ => panic!("Incorrect Token Type given to function")
    }
}

// make_string will concatinate anything between two double quotes. Will return
// an error if no closing double quote is given
fn make_string(index: &mut usize, words: &[&str]) -> Result<Token, ParserError> {
    *index += 1;
    let mut result_string = String::new();
    while *index < words.len() && words[*index] != "\"" {
        if result_string.is_empty() {
            result_string = words[*index].to_string();
        } else {
            result_string = vec![result_string, words[*index].to_string()].join(" ");
        }
        *index += 1;
    }
    if *index < words.len() {
        Ok(Token::String(result_string))
    } else {
        Err(ParserError::IncompleteString)
    }
}


fn make_if_expression(index: &mut usize, words: &[&str], stack: &mut VecDeque<Token>) -> Result<Token, ParserError> {
    *index += 1;
    for _ in 0..2 {
        let token = match get_token(index, words, stack)? {
            Token::Block(x) => Token::Block(x),
            x => Token::Block(vec![x])
        };
        *index += 1;
        stack.push_back(token);
    }
    Ok(Token::Symbol("if".to_string()))

}

fn make_binary_infix_to_postfix(index: &mut usize, words: &[&str], stack: &mut VecDeque<Token>) -> Result<Token, ParserError> {
    let op = words[*index];
    *index += 1;
    let token = get_token(index, words, stack)?;
    stack.push_back(token);
    Ok(Token::Symbol(op.to_string()))
}

fn make_binary_prefix_to_postfix(index: &mut usize, words: &[&str], stack: &mut VecDeque<Token>) -> Result<Token, ParserError> {
    let op = words[*index];
    *index += 1;
    let token = get_token(index, words, stack)?;
    stack.push_back(token);
    *index += 1;
    let token = get_token(index, words, stack)?;
    stack.push_back(token);
    Ok(Token::Symbol(op.to_string()))
}
