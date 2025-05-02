//! Entry point for the C4 Rust compiler.
//! Runs the lexer, parser, and virtual machine on a sample expression.

mod lexer;
mod parser;
mod vm;
mod error;

use lexer::Lexer;
use parser::{Parser, Expr};
use vm::{VM, Instruction};
use error::CompileError;

/// Main execution flow: lex, parse, compile, and run an arithmetic expression.
/// Returns `Ok(())` on success or a `CompileError` if any stage fails.
fn main() -> Result<(), CompileError> {
    let source = "1 + 2 * 3";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?; 
    let expr = parser.parse_expr()?;      

    // Compile to VM instructions
    let code = compile_expr(expr);
    let mut vm = VM::new(code);
    let result = vm.run();
    println!("Result: {}", result);
    Ok(())
}

/// Recursively compiles an expression into a vector of virtual machine instructions.
/// # Arguments
/// * `expr` - An abstract syntax tree representing the parsed expression.
/// # Returns
/// A vector of `Instruction`s to be executed by the VM.
fn compile_expr(expr: Expr) -> Vec<Instruction> {
    match expr {
        Expr::Number(n) => vec![Instruction::Push(n)],
        Expr::BinaryOp(left, op, right) => {
            let mut code = compile_expr(*left);
            code.extend(compile_expr(*right));
            match op {
                lexer::Token::Add => code.push(Instruction::Add),
                lexer::Token::Sub => code.push(Instruction::Sub),
                lexer::Token::Mul => code.push(Instruction::Mul),
                lexer::Token::Div => code.push(Instruction::Div),
                _ => panic!("Unsupported operator"),
            }
            code
        }
        Expr::Variable(name) => {
            panic!("Variable '{}' not supported in this stage", name);
        }
    }
}
