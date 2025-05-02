pub enum Instruction {
    Push(i64),
    Add,
    Sub,
    Mul,
    Div,
    // ... (other instructions)
}

pub struct VM {
    stack: Vec<i64>,
    code: Vec<Instruction>,
    pc: usize, // Program counter
}

impl VM {
    pub fn new(code: Vec<Instruction>) -> Self {
        VM { stack: Vec::new(), code, pc: 0 }
    }

    pub fn run(&mut self) -> i64 {
        while self.pc < self.code.len() {
            match self.code[self.pc] {
                Instruction::Push(val) => self.stack.push(val),
                Instruction::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);
                }
                // ... (other instructions)
                _ => panic!("Unknown instruction"),
            }
            self.pc += 1;
        }
        self.stack.pop().unwrap_or(0)
    }
}
