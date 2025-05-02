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
                Instruction::Mul => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b * a);
                }
                Instruction::Div => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b / a);
                }
                // ... (other instructions)
    
            }
            self.pc += 1;
        }
        self.stack.pop().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_execution() {
        let code = vec![
            Instruction::Push(2),
            Instruction::Push(3),
            Instruction::Mul,
            Instruction::Push(4),
            Instruction::Add,
        ];
        let mut vm = VM::new(code);
        let result = vm.run();
        assert_eq!(result, 10); // (2 * 3) + 4 = 10
    }

    #[test]
    fn test_division_and_subtraction() {
        let code = vec![
            Instruction::Push(20),
            Instruction::Push(5),
            Instruction::Div,
            Instruction::Push(2),
            Instruction::Sub,
        ];
        let mut vm = VM::new(code);
        let result = vm.run();
        assert_eq!(result, 2); // (20 / 5) - 2 = 2
    }
}
