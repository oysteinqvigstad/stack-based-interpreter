### Functional requirements.

**1. Parsing:**

- [x] Parse integers, floats, strings, booleans, lists, quotations, and symbols
- [x] Handle parser errors (*IncompleteString*, *IncompleteList*, *IncompleteQuotation*)

**2. Interpreter operations:**

a. Stack operations:

- [x] Implement `swap`, `dup`, and `pop`

b. IO operations:

- [x] Implement `print` and `read`

c. Runtime string parsing:

- [x] Implement `parseInteger`, `parseFloat`, and `words`

d. Arithmetic and boolean operations:

- [x] Handle integer and float arithmetic

  comment: `uses type coercion`

- [x] Implement boolean operations (`&&`, `||`, `not`)

e. List operations:

- [x] Implement `head`, `tail`, `empty`, `length`, `cons`, and `append`

f. Control flow:

- [x] Handle quotations
- [x] Implement `exec`, `if`, `times`, and `loop`
- [x] Implement `map`, `foldl`, and `each` for control flow with lists

g. Variables and functions:

- [x] Handle assignment and function definition with `:=` and `fun`
- [x] Evaluate and execute bound symbols (variables and functions)
- [x] Implement `'` (tick) operator to put a bound symbol onto the stack
- [ ] Implement `eval` function to evaluate bound symbols to values

h. Error handling:

- [x] Implement and handle program execution errors (*StackEmpty*, *DivisionByZero*, *ExpectedList*, etc.)
- [x] Report errors to the user with meaningful messages

**3. Program execution:**

- [x] Ensure that the interpreter produces a single value on top of the value stack upon program completion
- [x] Offer two modes of operation, `REPL` and `NORMAL`

**4. I/O:**

- [x] Implement `print` and `read` operations within the interpreter
- [x] Separate I/O from core interpreter logic to support automated testing
- [x] Represent the stack using compact pretty notation

**5. Automated testing (excluding I/O):**

- [x] Implement automated tests for all interpreter operations and features except print and read operations

### Non-functional requirements.

**Performance:**

- [x] The interpreter is efficient and executes programs quickly

- [x] The tests should not take a maximum of 1 seconds to complete.

  comment: `takes ~500 ms`

**Error handling:**

- [x] Clear and helpful error messages during parsing and execution have been provided
- [x] The program does not appear to crash easily

**Maintainability:**

- [x] The code is organized well
- [x] The code is modular
- [x] It is easy to extend features
- [x] The code is easy to read

**Usability:**

- [x] REPL mode is implemented and simple to use and understand

  comment: `no :help command `

- [x] There are no restrictive libraries such that the program can compile on different operating systems

**Documentation:**

- [x] Testing report is included
- [x] Functional requirements are included
- [x] Non-functional requirements are included
- [x] Assessment specification is included
- [x] Self-assessment report is included
- [x] Instructions on how to run the program is included

**Testing:**

- [x] REPL seems to adhere to the specs
- [x] Test package including all the integration tests have been included
- [x] At least 80% of the test for the implemented operations have passed

**Professionalism:**

- [x] The commit messages are meaningful
- [x] A merge request have been made to the PROG2006 labs
- [x] Documentation is composed with a focus on thoroughness and professionalism



### Evaluation

| Criteria        | Weight | Evaluation                                                   | Score |
| --------------- | ------ | ------------------------------------------------------------ | ----- |
| Functionality   | 35%    | All except `eval` has been implemented                       | 34    |
| Documentation   | 20%    | All documents have been provided                             | 20    |
| Maintainability | 15%    | The code is easy to follow and modularized                   | 15    |
| Error handling  | 10%    | All error messages and some additional have been implemented | 10    |
| Usability       | 10%    | REPL and normal mode implemented. Would be helpful if a `help` command was provided | 8     |
| Professionalism | 5%     | Commit messages are meaningful. Could be more atomic         | 4     |
| Performance     | 5%     | No crashes detected and the tests perform within reasonable time | 5     |

Final Score: ` 96` out of 100



### Final reflection

I'm very happy with the outcome of this project, as I was able to finish on time and I believe the source code is very tidy, modularized and readable. There might be better constructions available but I found that using simple pattern matching and result propagation I was able to accomplish all that was required. 

I initially started with an abstract parse tree represented in a stack, which the interpreter would evaluate based on the type of Token how many children it would have or be considered as a leaf node. It would then use recursion and propagate each result to its own parent. The problem I had with this approach was that the function or variable definition might be in a completely different branch and so I could not execute it with a single pass. That's why I decided to refactor the code to use an instruction list and stack instead to keep track of the current state. Even though the refactoring from AST was very time consuming, and it felt like a personal loss, I think it was a good learning experience to initially start working with an AST, as it forced me to think about how to deal with tree structures, and corner cases. It also forced me to evaluate whether the approach was worthwhile to continue working on.

Furthermore I'm glad that it worked out well in Rust, and I was able to gain some experience in that language as well. I initially had some doubts, since the lecturer warned that it would be more difficult to do in Rust. 