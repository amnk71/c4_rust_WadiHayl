# C4 Rust Compiler – Team WadiHayl
Submitted by:
Amna Alotaiba 100053254
Noura Alshamsi 100060665

This is a Rust reimplementation of the original C4 compiler. It is capable of parsing and evaluating a subset of C-like expressions, using lexer, parser, and virtual machine backend.

## Features

- Parses and evaluates integer arithmetic expressions like : `1 + 2 * 3`
- Tokenizing keywords like `if`, `else`, and `return`
- Converts code into VM bytecode and executes it
- Graceful error handling for invalid syntax and unknown tokens
- **Bonus Feature:** Enhanced error reporting with custom error types and line tracking

---

## Build Instructions

To build the project, use Cargo:
cargo build
cargo run (general code run)
cargo test (testing unit cases)

---

# Bonus Feature: Enhanced Error Reporting

This Rust version introduces structured error handling using a custom CompileError enum, improving on the original C4's behavior. It provides detailed, line-aware feedback for:

Lexical errors (e.g., unknown tokens like @)
Parsing errors (e.g., unexpected end of input)
Future runtime errors (extensible)

Errors are caught early and displayed clearly, improving debugging and robustness.

Example:
Parser error on line 1: Unexpected token: "@"

Tests are included to ensure this behavior is consistent and reliable.

Bonus feature tested to work, is called test_invalid_token_error 
![image](https://github.com/user-attachments/assets/4ab74616-266d-4a1b-b5a6-ce52a2746a88)
