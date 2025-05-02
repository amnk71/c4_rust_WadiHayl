use std::fmt;

/// Represents all compile-time and runtime errors in the compiler.
#[derive(Debug, Clone)]
pub enum CompileError {
    /// Raised when the lexer encounters an unknown character or malformed input.
    Lexer {
        message: String,
        line: usize,
    },

    /// Raised when the parser encounters invalid syntax or an incomplete expression.
    Parser {
        message: String,
        line: usize,
    },

    /// Raised at runtime for issues like unhandled instructions or stack errors.
    Runtime {
        message: String,
    },
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Lexer { message, line } => {
                write!(f, "Lexer error on line {}: {}", line, message)
            }
            CompileError::Parser { message, line } => {
                write!(f, "Parser error on line {}: {}", line, message)
            }
            CompileError::Runtime { message } => {
                write!(f, "Runtime error: {}", message)
            }
        }
    }
}

impl std::error::Error for CompileError {}
