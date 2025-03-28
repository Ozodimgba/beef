use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Push,
    Pop,

    Add,
    Sub,
    Mul,
    Div,

    LoadReg, // Load from register to stack
    StoreReg, // Store from stack to register

    //TODO: Mem Ops and Control flow

    Exit 
}

// execution context
pub struct Context {
    pc: usize,

    stack: Vec<i64>, // LIFO stack here is just a logical concept not rust physical call stack

    registers: [i64; 11],

    memory: HashMap<usize, i64>,

    program: Vec<Instruction>
}

// Instruction structure
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operands: Vec<i64>,
}

impl Context {
    pub fn new(program: Vec<Instruction>) -> Self {
        Context { pc: 0, stack: Vec::new(), registers: [0; 11], memory: HashMap::new(), program}
    }

    pub fn run(&mut self) -> Result<i64, String> {
        while self.pc < self.program.len() {
            let instruction = self.program[self.pc].clone();

            //execute function
            self.execute_ix(instruction)?;

            if matches!(self.program[self.pc].opcode, OpCode::Exit){
                return Ok(self.registers[0]);
            }
        }

        Err("Program terminated without explicit exit".to_string())
    }

    fn execute_ix(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction.opcode {
            OpCode::Push => {
                if instruction.operands.is_empty() {
                    return Err("Push requires an operand".to_string())
                }
                self.stack.push(instruction.operands[0]);
                self.pc += 1;
            },
            OpCode::Pop => {
                self.stack.pop().ok_or("Stack Underflow")?;
                self.pc += 1;
            }
            OpCode::Add => {
                let b = self.stack.pop().ok_or("Stack Underflow")?;
                let a = self.stack.pop().ok_or("Stack Underflow")?;

                self.stack.push(a + b);

                self.pc += 1;
            },
            OpCode::Sub => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                self.stack.push(a - b);
                self.pc += 1;
            },          
            OpCode::Mul => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                self.stack.push(a * b);
                self.pc += 1;
            },            
            OpCode::Div => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                let a = self.stack.pop().ok_or("Stack underflow")?;
                self.stack.push(a / b);
                self.pc += 1;
            },

            //register operations
            OpCode::LoadReg => {
                if instruction.operands.is_empty() {
                    return Err("LoadReg requires a register index operand".to_string());
                }
                let reg_idx = instruction.operands[0] as usize;
                if reg_idx >= self.registers.len() {
                    return Err(format!("Invalid register index: {}", reg_idx));
                }
                self.stack.push(self.registers[reg_idx]);
                self.pc += 1;
            },
            OpCode::StoreReg => {
                if instruction.operands.is_empty() {
                    return Err("StoreReg requires a register index operand".to_string());
                }
                let reg_idx = instruction.operands[0] as usize;
                if reg_idx >= self.registers.len() {
                    return Err(format!("Invalid register index: {}", reg_idx));
                    let value = self.stack.pop().ok_or("Stack Overflow")?;
                    self.registers[reg_idx] = value;
                    self.pc += 1;
                }
            },
            OpCode::Exit => {
                return Ok(());
            }
        }

        Ok(())
    }
}

fn main() {
    print!("main")
}