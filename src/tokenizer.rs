use crate::stack::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut index = 0;

    let words = lex(input);

    println!("{}", is_function(&"+".to_string()));

    while index < words.len() {
        let token = match words[index] {

            s if is_bool(s) => Token::Bool(s.to_lowercase().parse::<bool>().unwrap()),
            s if is_integer(s) => Token::Int(s.parse::<i64>().unwrap()),
            s if is_float(s) => Token::Float(s.parse::<f32>().unwrap()),
            s if is_function(s) => Token::Operation(s.to_string()),
            s => Token::String(s.to_string())
        };
        tokens.push(token);
        index += 1;
    }


    println!("{:?}", tokens);

    vec![]
}

fn lex(input: &str) -> Vec<&str> {
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
    let ops = vec!["+", "-", "/", "*"];
    ops.contains(&s)
}