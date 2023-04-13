use prog2006_assignment_2::{parse, execute};

fn main() {
    let res = parse("1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }");
    // 1 { dup 4 > } { dup 1 + } loop [ ] cons cons cons cons cons


    // println!("{}", Token::List(vec![Token::Bool(true), Token::Int(5)]));
    // println!("{:?}", parse("True [ False ] 5 25.2"));
    println!("{:?}", res);
    match res {
        Ok(x) => {
            let mut test = x;
            println!("Stack printed neatly: {}", &test);
            println!("Result: {:?}", execute(&mut test));
        },
        Err(x) => println!("{:?}", x)
    }
}
