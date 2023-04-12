use prog2006_assignment_2::{parse, execute};

fn main() {
    let mut res = parse("[ ] empty");
    // println!("{}", Token::List(vec![Token::Bool(true), Token::Int(5)]));
    // println!("{:?}", parse("True [ False ] 5 25.2"));
    println!("{:?}", res);
    match res {
        Ok(x) => {
            let mut test = x;
            println!("{}", &test);
            println!("{:?}", execute(&mut test));
        },
        Err(x) => println!("{:?}", x)
    }
}
