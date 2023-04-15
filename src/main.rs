use prog2006_assignment_2::{parse, execute};

fn main() {
    let res = parse("age 10 := age");
    match res {
        Ok(x) => {
            let mut test = x;
            println!("Before running: {:?}", &test);
            println!("Result: {:?}", execute(&mut test));
        },
        Err(x) => println!("{:?}", x)
    }
}

// "1 loop { dup 4 > } { dup 1 + }      [ ] 5 times { cons }"
// "1 True
// 1 2





// 1
//