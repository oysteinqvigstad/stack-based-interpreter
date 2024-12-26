[TOC]

# Instructions

### How to start the program

The application have two modes. `NORMAL` and `REPL` mode.

In Normal mode, the program reads input from the standard input (stdin), which can be redirected from a command or file. This mode is ideal for processing predetermined data or commands. On the other hand, REPL mode offers an interactive environment where users can enter expressions and statements during a live session, while maintaining the state of previous interactions. REPL mode is more lenient with errors, allowing for recovery and providing information about the state of the program's stack at each step of the process.

#### 1. Using NORMAL mode

Normal mode is designed to accept input that has been redirected from another program or from a file. In both cases, the program processes the input, writes the result to the standard output (stdout), and then terminates.

To start Normal mode using cargo and read input from a file named expression.txt, execute the following command from your project's root directory:


```bash
cargo run < expression.txt
```

If you prefer to parse redirected output from another program, use the command below:

```bash
echo "1 2 + 3 ==" | cargo run
```

This command first runs the echo command, which outputs the specified expression. Then, the output is piped to cargo run, allowing your program to process the expression and display the result.


#### 2. Using REPL-mode

To initiate REPL mode using Cargo, execute the following command:

```bash
cargo run -- repl
```
This command launches an interactive REPL session, allowing you to work with your code in real-time and maintain the state of your previous interactions. Upon entering REPL mode, you will be presented with an interactive prompt for user input, as shown below:

```
bprog >
```

The stack is represented from left to right, with the rightmost element being the top of the stack.

In addition to entering expressions and statements, you can use the following commands to manage and control your REPL session:

- `:b` Displays a list of active bindings, which are the current variable assignments in the session.
- `:f` Shows a list of active function definitions, providing an overview of the functions that have been bound
- `:q` Exits the REPL session, allowing you to return to the command prompt or terminal.

These commands offer convenient ways to explore and manage your REPL environment while working with your code interactively.

##### REPL-mode Example

You can now interactively add literals and operations to your program in the REPL mode:

```
bprog > 1 4 swap [ 2 3 ] cons
```

1. the numbers `1` and `4` are pushed onto the stack. 
2. The `swap` operation reverses the order of the top two elements on the stack. 
3. The list literal `[2,3]` is then added to the stack. 
4. The `cons` operation takes the list (`[2,3]`) and the next element (`1`) from the stack, and prepends the latter to the list, resulting in the following stack:

```
stack : 4 [1,2,3]
warn  : ProgramFinishedWithMultipleValues
```

You can continue to interact with the existing stack in the REPL mode:

```
bprog > map { 2 * }
```

This command maps each element in the list and multiplies it by `2`, returning a new list:

```
stack : 4 [2,4,6]
warn  : ProgramFinishedWithMultipleValues
```

Now, let's use the `foldl` operation:

```
bprog > swap foldl { + }
```

This command performs the following actions:

1. The `swap` operation reverses the elements `4` and `[2,4,6]`. 
2. The `foldl` operation uses `4` as the accumulator and adds the elements `2`, `4`, and `6` to it.

The final stack looks like this:

```
stack : 16
```



# Project Structure



    ├── Cargo.toml             // Package configuration
    ├── src	
    │   ├── error.rs           // Enum definitions of errors
    │   ├── interpreter.rs     // Main execution logic for a given instruction set
    │   ├── lib.rs             // Entry point for REPL, Testing, etc.
    │   ├── main.rs            // Entry point for the application
    │   ├── parser.rs          // Lexer and Parser
    │   ├── state.rs           // Definition of stack, instructions and bindings
    │   └── token.rs           // Token struct and methods for most operations
    └── tests					
        └── tests.rs           // Integration tests



# Functional Requirements

1. Parsing:
   - Parse integers, floats, strings, booleans, lists, quotations, and symbols
   - Handle parser errors (*IncompleteString*, *IncompleteList*, *IncompleteQuotation*)

2. Interpreter operations:

    a. Stack operations:

    - Implement `swap`, `dup`, and `pop`

    b. IO operations:

    - Implement `print` and `read`

    c. Runtime string parsing

    - Implement `parseInteger`, `parseFloat` and `words`

    d. Arithmetic and boolean operations:

    - Handle integer and float arithmetic
    - Implement boolean operations (`&&`, `||`, `not`)

    e. List operations:

    - Implement `head`, `tail`, `empty`, `length`, `cons`, and `append`

    f. Control flow:

    - Handle quotations
    - Implement `exec`, `if`, `times`, and `loop`
    - Implement `map`, `foldl`, and `each` for control flow with lists

    g. Variables and functions:

    - Handle assignment and function definition with `:=` and `fun`
    - Evaluate and execute bound symbols (variables and functions)
    - Implement `'` (tick) operator to put a bound symbol onto the stack
    - Implement `eval` function to evaluate bound symbols to values

    h. Error handling:

    - Implement and handle program execution errors (*StackEmpty*, *DivisionByZero*, *ExpectedList*, etc.)
    - Report errors to the user with meaningful messages

3. Program execution:
    - Ensure that the interpreter produces a single value on top of the value stack upon program completion
    - Offer two modes of operation, `REPL` and `NORMAL`

4. I/O:
    - Implement `print` and `read` operations within the interpreter
    - Separate I/O from core interpreter logic to support automated testing
    - Represent the stack using compact pretty notation

5. Automated testing (excluding I/O):
    - Implement automated tests for all interpreter operations and features except print and read operations 
