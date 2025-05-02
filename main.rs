mod lexer;
mod parser;
mod vm;
mod error;

use lexer::Lexer;
use parser::{Parser, Expr};
use vm::{VM, Instruction};
use error::CompileError;

fn main() -> Result<(), CompileError> {
    let source = "1 + 2 * 3";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?; 
    let expr = parser.parse_expr()?;      
    
    // Compile to VM instructions
    let code = compile_expr(expr);
    let mut vm = VM::new(code);
    let result = vm.run();
    println!(" Result: {}", result);
    Ok(())
}

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
            panic!(" Variable '{}' not supported in this stage", name);
        }
    }
}
