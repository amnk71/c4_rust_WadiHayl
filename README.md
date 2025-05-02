# c4_rust_WadiHayl 
Submitted by:
Amna Alotaiba 100053254
Noura Alshamsi 100060665

*Bonus Feature: Enhanced Error Reporting*

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
