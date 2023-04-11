use crate::enums::Token;
use crate::enums::ParserError;
use crate::stack::Stack;



pub fn parse(s: &str) -> Result<Stack, ParserError> {
    tokenize_and_parse(&lex(s))
}

// tokenize breaks down a string into a Stack of tokens
pub fn tokenize_and_parse(words: &[&str]) -> Result<Stack, ParserError> {
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < words.len() {
        let token = match words[index] {
            "[" => make_collection(&mut index, words, Token::List(vec![]))?,
            "{" => make_collection(&mut index, words, Token::Block(vec![]))?,
            "\"" => make_string(&mut index, words)?,
            "]" => Err(ParserError::IncompleteList)?,
            "}" => Err(ParserError::IncompleteQuotation)?,
            s if is_bool(s) => Token::Bool(s.to_lowercase().parse::<bool>().unwrap()),
            s if is_integer(s) => Token::Int(s.parse::<i64>().unwrap()),
            s if is_float(s) => Token::Float(s.parse::<f32>().unwrap()),
            s if is_function(s) => Token::Operation(s.to_string()),
            s => Token::String(s.to_string())

        };
        tokens.push(token);
        index += 1;
    }


    Ok(Stack{tokens})
}

pub fn lex(input: &str) -> Box<[&str]> {
    let vec: Vec<&str> = input.split_whitespace().collect();
    vec.into_boxed_slice()
    // let vec: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();
    // slice
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

// is_function checks if the token is a function that reads other tokens
fn is_function(s: &str) -> bool {
    let arithmetic = vec!["+", "-", "*", "/", "div"];
    let logical = vec!["<", ">", "==", "&&", "||", "not"];
    let list = vec!["head", "tail", "empty", "length", "cons", "append", "each", "map", "foldl"];
    // TODO: Control flow?
    arithmetic.contains(&s) || logical.contains(&s) || list.contains(&s)
}

// make_collection ensures that everything between [] or {} is parsed as token that holds
// a vector of other tokens. The function can also construct nested lists/blocks. If a
// list/block is not closed then the function will return an error.
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
        (Token::List(_), 0)  => Ok(Token::List(tokenize_and_parse(&words[start_index..*index])?.tokens)),
        (Token::Block(_), 0) => Ok(Token::Block(tokenize_and_parse(&words[start_index..*index])?.tokens)),
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