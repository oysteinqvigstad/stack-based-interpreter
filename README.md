# Instructions

### How to start the program

In the project root, type the following command to build and run the program:

```bash
cargo run
```



### How to use the program

Once in `REPL` mode, you should be greeted with the following prompt for interaction:

```
bprog >
```

The stack is expressed from left to right, the rightmost being the top element on the stack.

#### Example:

You can now add literals and operations to the program:

```
bprog > 1 4 swap [ 2 3 ] cons
```

This should parse and execute the following steps:

1. `1` and `4` are put on the stack.
2. `swap` reverses the two element on the stack
3. `[2,3]` list literal is put onto the stack
4. `cons` takes the list (`[2,3]`) and next element (`1`) from the stack and prepends the last one to the list, like this:

```
stack : 4 [1,2,3]
warn  : ProgramFinishedWithMultipleValues
```

We can continue to interact with the existing stack with `REPL`:

```
bprog > map { 2 * }
```

This will map each element in the list and multiply it with`2` and return a new list

```
stack : 4 [2,4,6]
warn  : ProgramFinishedWithMultipleValues
```

Lets use `foldl`:

```
bprog > swap foldl { + }
```

This will effectively:

1. `swap` will reverse the elements `4` and `[2,4,6]`
2. with `foldl`, `4` is used as accumulator, and will add `2 + 4 + 6` onto `4`

```
stack : 16
```



# Project Structure



    ├── Cargo.toml				// Package configuration
    ├── src	
    │   ├── interpreter.rs		// Main execution logic for a given instruction set
    │   ├── lib.rs				// Entry point for REPL, Testing, etc.
    │   ├── main.rs				// Entry point for the application
    │   ├── parser.rs			// Lexer and Parser
    │   ├── state.rs			// Definition of stack, instructions and bindings
    │   └── token.rs			// Token struct and methods for most operations
    └── tests					
    	└── tests.rs			// Integration tests



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

    d. List operations:
    - Implement `head`, `tail`, `empty`, `length`, `cons`, and `append`

    e. Control flow:
    - Handle quotations
    - Implement `exec`, `if`, `times`, and `loop`
    - Implement `map`, `foldl`, and `each` for control flow with lists

    f. Variables and functions:
    - Handle assignment and function definition with `:=` and `fun`
    - Evaluate and execute bound symbols (variables and functions)
    - Implement `'` (tick) operator to put a bound symbol onto the stack
    - Implement `eval` function to evaluate bound symbols to values

    f. Error handling:
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

# Non-functional Requirements

**Performance**: The interpreter should be efficient and execute programs as quickly as possible.

**Error handling**: The interpreter should provide clear and helpful error messages when errors occur, both during parsing and execution.

**Maintainability**: The code should be well-organized, modular, and easy to modify and extend as needed.

**Usability**: The interpreter should be easy to use and have a user-friendly interface for inputting and running programs.

**Compatibility**: The interpreter should be compatible with a wide range of operating systems and environments. The interpreter should be easily portable to different platforms and architectures, and should support cross-compilation if needed.

**Documentation**: The application must be documented, with a report of integration testing, functional requirements, non-functional requirements, and a assessment specification. Based on the assessment specification It should also include a self-assessment report. Short instructions on how to run the program should also be added.

**Testing**: The interpreter should be thoroughly tested with the provided integration tests to ensure correct behavior. 

# Assessment Specifications

The application should:

### Meet most of the functional requirements.

- [ ] **1. Parsing:**
  - [ ] Parse integers, floats, strings, booleans, lists, quotations, and symbols
  - [ ] Handle parser errors (*IncompleteString*, *IncompleteList*, *IncompleteQuotation*)
- [ ] **2. Interpreter operations:**
  - [ ] a. Stack operations:
   - [ ] Implement `swap`, `dup`, and `pop`
  - [ ] b. IO operations:
    - [ ] Implement `print` and `read`
  - [ ] c. Runtime string parsing:
    - [ ] Implement `parseInteger`, `parseFloat`, and `words`
  - [ ] d. Arithmetic and boolean operations:
    - [ ] Handle integer and float arithmetic
    - [ ] Implement boolean operations (`&&`, `||`, `not`)
  - [ ] e. List operations:
    - [ ] Implement `head`, `tail`, `empty`, `length`, `cons`, and `append`
  - [ ] f. Control flow:
    - [ ] Handle quotations
    - [ ] Implement `exec`, `if`, `times`, and `loop`
    - [ ] Implement `map`, `foldl`, and `each` for control flow with lists
  - [ ] g. Variables and functions:
    - [ ] Handle assignment and function definition with `:=` and `fun`
    - [ ] Evaluate and execute bound symbols (variables and functions)
    - [ ] Implement `'` (tick) operator to put a bound symbol onto the stack
    - [ ] Implement `eval` function to evaluate bound symbols to values
  - [ ] h. Error handling:
    - [ ] Implement and handle program execution errors (*StackEmpty*, *DivisionByZero*, *ExpectedList*, etc.)
    - [ ] Report errors to the user with meaningful messages
- [ ] **3. Program execution:**
  - [ ] Ensure that the interpreter produces a single value on top of the value stack upon program completion
  - [ ] Offer two modes of operation, `REPL` and `NORMAL`
- [ ] **4. I/O:**
  - [ ] Implement `print` and `read` operations within the interpreter
  - [ ] Separate I/O from core interpreter logic to support automated testing
  - [ ] Represent the stack using compact pretty notation
- [ ] **5. Automated testing (excluding I/O):**
  - [ ] Implement automated tests for all interpreter operations and features except print and read operations

### Meet all of the non-functional requirements.

- [ ] **Performance:**
    - [ ] The interpreter is efficient and executes programs quickly


- [ ] **Error handling:**
  - [ ] Clear and helpful error messages during parsing and execution have been provided
- [ ] **Maintainability:**
  - [ ] The code is organized well
  - [ ] The code is modular
  - [ ] It is easy to extend features
- [ ] **Usability:**
  - [ ] REPL is implemented and simple to use and understand
- [ ] **Compatibility:**
  - [ ] There are no restrictive libraries such that the program can compile on different operating systems
- [ ] **Documentation:**
  - [ ] Testing report is included
  - [ ] Functional requirements are included
  - [ ] Non-functional requirements are included
  - [ ] Assessment specification is included
  - [ ] Self-assessment report is included
  - [ ] Instructions on how to run the program is included
- [ ] **Testing:**
  - [ ] REPL seems to adhere to the specs
  - [ ] Test package including all the integration tests have been included
    - [ ] At least 80% of the test for the implemented operations have passed

### Maintain professionalism

- [ ] The commit messages are meaningful
- [ ] A merge request have been made to the PROG2006 labs
- [ ] Documentation is composed with a focus on thoroughness and professionalism

