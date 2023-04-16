use prog2006_assignment_2::repl_mode;

fn main() {
    repl_mode();
}

//"[True,False,True,False,True]");

// odd { dup 2 div swap 2 / == if False True } fun toList { [ ] swap times cons } fun gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun 4 gen1toNum 5 toList map odd")

// odd { dup 2 div swap 2 / == if False True } fun
// toList { [ ] swap times cons } fun
// gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun 4 gen1toNum 4 toList map odd

// 4 gen1ToNum
// 1 2 3 4

// 1 2 3 4 [ ] 5 times cons

// [ 1 2 3 4 5 ] 5 [ ] swap times cons
// [ 1 2 3 4 5 ] [ ] 5 times cons

// 1 2 3 4 5 5
// [ ] swap times cons