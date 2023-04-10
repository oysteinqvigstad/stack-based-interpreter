use std::fs::copy;
use std::ptr::null;
use crate::stack::Token;
use crate::stack::Stack;
use crate::stack::Token::List;
use crate::stack::Token::Block;
use crate::stack::Token::String;

pub fn tokenize(words: &[&str]) -> Stack {
    let mut tokens = Vec::new();
    let mut index = 0;


    let mut test = 4;

    while index < words.len() {
        let token = match words[index] {
            "[" => make_collection(&mut index, words, List(vec![])),
            "{" => make_collection(&mut index, words, Block(vec![])),
            s if is_bool(s) => Token::Bool(s.to_lowercase().parse::<bool>().unwrap()),
            s if is_integer(s) => Token::Int(s.parse::<i64>().unwrap()),
            s if is_float(s) => Token::Float(s.parse::<f32>().unwrap()),
            s if is_function(s) => Token::Operation(s.to_string()),
            s => Token::String(s.to_string())
        };
        tokens.push(token);
        index += 1;
    }


    Stack{tokens}
}

pub fn lex(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

// is_integer returns true if the entire token is an integer. Also checks for negative numbers
fn is_integer(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-'));
    return at_least_one_digit && rest_are_legal
}

// is_integer returns true if the entire token is a float. Also checks for negative numbers
fn is_float(s: &str) -> bool {
    let at_least_one_digit = s.chars().any(|c| c.is_digit(10));
    let exactly_one_dot = s.chars().filter(|c| *c == '.').count() == 1;
    let rest_are_legal = s.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 0 && c == '-') || c == '.');
    at_least_one_digit && exactly_one_dot && rest_are_legal
}

fn is_bool(s: &str) -> bool {
    s == "true" || s == "True" || s == "false" || s == "False"
}

fn is_function(s: &str) -> bool {
    let arithmatic = vec!["+", "-", "*", "/", "div"];
    let logical = vec!["<", ">", "==", "&&", "||", "not"];
    let list = vec!["head", "tail", "empty", "length", "cons", "append", "each", "map", "foldl"];
    // TODO: Control flow?
    arithmatic.contains(&s) || logical.contains(&s) || list.contains(&s)
}

fn make_list(index: &mut usize, words: &[&str]) -> Token {
    let list: Vec<Token> = Vec::new();
    let mut list_level = 0;
    let start_index = *index + 1;
    while *index < words.len() {
        list_level += match words[*index] {
            "[" =>  1,
            "]" => -1,
            _   =>  0
        };

        if list_level == 0 {
            break;
        }

        *index += 1;
    }
    if list_level != 0 {
        panic!("unmatching braces");
    }
    Token::List(tokenize(&words[start_index..*index]).tokens)

}

fn make_collection(index: &mut usize, words: &[&str], t: Token) -> Token {
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
    if level != 0 {
        panic!("unmatching braces");
    }


    match t {
        List(_) => Token::List(tokenize(&words[start_index..*index]).tokens),
        Block(_) => Token::Block(tokenize(&words[start_index..*index]).tokens),
        _ => Token::Block(tokenize(&words[start_index..*index]).tokens),
    }
}

    // while *index < words.len() {
    //     match t {
    //         List(_) => level += match words[*index] {
    //                                         "[" =>  1,
    //                                         "]" => -1,
    //                                         _   =>  0
    //                                 },
    //         Block(_) => level += match words[*index] {
    //             "{" =>  1,
    //             "}" => -1,
    //             _   =>  0
    //         },
    //         _ => panic!("should not have been invocated")
    //     }
    //
    //     if level == 0 {
    //         break;
    //     }
    //
    //     *index += 1;
    // }
    // if level != 0 {
    //     panic!("unmatching braces");
    // }
    // println!("{}", *index);
    // match t {
    //     List(_) => Token::List(tokenize(&words[start_index..*index]).tokens),
    //     Block(_) => Token::Block(tokenize(&words[start_index..*index]).tokens),
    //     _ => panic!("should not have been invocated")
    // }
// }