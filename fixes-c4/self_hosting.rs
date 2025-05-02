use c4_rust_WadiHayl::*;
use std::fs;

#[test]
fn self_hosting_c4_c() {
    let source = fs::read_to_string("fixtures/c4.c").expect("Failed to read c4.c");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    // These would be real functions  implemented
    let program = parser.parse_program();      
    let bytecode = compile_program(program);   

    let mut vm = VM::new(bytecode);
    let exit_code = vm.run();

    // Assuming successful exit is 0
    assert_eq!(exit_code, 0);
}
