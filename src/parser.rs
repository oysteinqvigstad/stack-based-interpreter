use crate::token::Token;
use crate::state::State;

/// Error types that may propagate during parsing
#[derive(Debug)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation
}

/// Entry point for the parser
///
/// The parser borrows a mutable state and makes changes to it.
/// It will return Ok(()) if the parsing is successful, otherwise
/// a ParseError is returned
///
/// # Arguments
///
/// * `input_string` - The string to be lexed and parsed
/// * `state` - The stack, instruction list, list of functions and bindings
///
/// # Errors
///
/// Returns ParseError if the operation fails during the tokenization
///
pub fn parse_string_to_instructions(input_string: &str, state: &mut State) -> Result<(), ParserError> {
    tokenize_and_parse(&lex(input_string), state)
}


/// Lexer
///
/// The lexer takes a string and separates it by whitespace, and returns a slice to
/// a heap allocated array of words. We want to return a slice because the parser
/// work by dynamically adjusting the slices when making collections
///
/// # Arguments
///
/// * `input_string` - The string to be lexed and parsed
///
pub fn lex(input_string: &str) -> Box<[&str]> {
    let vec: Vec<&str> = input_string.split_whitespace().collect();
    vec.into_boxed_slice()
}

/// Initiates the parsing phase
///
/// The parser borrows a mutable state and makes changes to it.
/// It will return Ok(()) if the parsing is successful, otherwise
/// a ParseError is returned
///
/// # Arguments
///
/// * `words` - The lexed input (input string expressed as a list of words)
/// * `state` - The stack, instruction list, list of functions and bindings
///
/// # Errors
///
/// Returns ParseError if the operation fails during the tokenization
///
fn tokenize_and_parse(words: &[&str], state: &mut State) -> Result<(), ParserError> {
    let mut index: usize = 0;

    while index < words.len() {
        let token = get_token(&mut index, words)?;
        state.instruction_set.push_back(token);
        index += 1;
    }
    Ok(())
}




/// Generate a token based on a word
///
/// Combines a slice with a borrowed index, so that it can easily skip ahead
/// after making a collection or multi worded string
///
/// # Arguments
///
/// * `index` - The borrowed index that mutates during execution
/// * `words` - The lexed input (input string expressed as a list of words)
///
/// # Errors
///
/// Returns ParseError if the operation fails during the tokenization
///
fn get_token(index: &mut usize, words: &[&str]) -> Result<Token, ParserError> {
    match words[index.clone()] {
        "[" => make_collection(index, words, Token::List(vec![])),
        "{" => make_collection(index, words, Token::Block(vec![])),
        "\"" => make_string(index, words),
        "]" => Err(ParserError::IncompleteList),
        "}" => Err(ParserError::IncompleteQuotation),
        s if is_bool(s) => Ok(Token::Bool(s.to_lowercase().parse::<bool>().unwrap())),
        s if is_integer(s) => Ok(Token::Int(s.parse::<i128>().unwrap())),
        s if is_float(s) => Ok(Token::Float(s.parse::<f32>().unwrap())),
        s => Ok(Token::Symbol(s.to_string()))
    }
}




/// Checks whether the word is a integer representation
///
/// # Arguments
///
/// * `s` - The lexed word to be evaluated
///
fn is_integer(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-'));
    return at_least_one_digit && rest_are_legal
}

/// Checks whether the word is a float representation
///
/// # Arguments
///
/// * `s` - The lexed word to be evaluated
///
fn is_float(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let exactly_one_dot = s.chars().filter(|c| *c == '.').count() == 1;
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-') || c == '.');
    at_least_one_digit && exactly_one_dot && rest_are_legal
}

/// Checks whether the word is a bool representation
///
/// # Arguments
///
/// * `s` - The lexed word to be evaluated
///
fn is_bool(s: &str) -> bool {
    s == "true" || s == "True" || s == "false" || s == "False"
}

/// Creates a collection of type Token::List or Token::Block
///
/// Reads ahead for closing brackets and calls the tokenizer on the slice
/// in between, so that the individual tokens can be inserted into the list.
///
/// # Arguments
///
/// * `index` - Mutable index so that when the collection is made the program
///             can continue at the updated index
/// * `words` - Slice of lexed input
/// * `t` - Type of collection, list or block
///
/// # Errors
///
/// Returns ParseError if there are missing or misplaced brackets/braces
///
fn make_collection(index: &mut usize, words: &[&str], t: Token) -> Result<Token, ParserError> {
    let mut collection_state = State::new();
    let mut level = 0;
    // keeps track of the opening and closing index
    let start_index = *index + 1;
    while *index < words.len() {
        // counts up the number of opening and closing brackets/braces
        level += match (&t, words[*index]) {
            (Token::List(_),  "[") =>  1,
            (Token::List(_),  "]") => -1,
            (Token::Block(_), "{") =>  1,
            (Token::Block(_), "}") => -1,
            _                      =>  0,
        };

        if level == 0 {
            // closing bracket/brace found
            break;
        } else {
            // continue
            *index += 1;
        }
    }
    // make the collection type and tokenize anyting in between
    match (t, level) {
        (Token::List(_), 0)  => {
            tokenize_and_parse(&words[start_index..*index], &mut collection_state)?;
            Ok(Token::List(collection_state.get_instructions()))
        },
        (Token::Block(_), 0) => {
            tokenize_and_parse(&words[start_index..*index], &mut collection_state)?;
            Ok(Token::Block(collection_state.get_instructions()))
        },
        // the index reached the end of the string and the closing bracket/brace was not found
        (Token::List(_), _)  => Err(ParserError::IncompleteList),
        (Token::Block(_), _) => Err(ParserError::IncompleteQuotation),
        _ => panic!("Incorrect Token Type given to function")
    }
}

/// Creates a string of type Token::String
///
/// Reads ahead for closing double quote generates a string of the characters in between
///
/// # Arguments
///
/// * `index` - Mutable index so that when the collection is made the program
///             can continue at the updated index
/// * `words` - Slice of lexed input
///
/// # Errors
///
/// Returns ParseError if there are missing or misplaced brackets/braces
///
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


