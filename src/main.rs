use prog2006_assignment_2::{parse, execute};

fn main() {
    let res = parse("[ 1 2 3 4 ] 0 foldl +");
    // 2 10 times 1 + times
    // 5 10 times +

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
