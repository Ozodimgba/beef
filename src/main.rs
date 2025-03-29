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

    //Mem Ops
    Load,
    Store,

    // control flow
    Jump,
    JumpEq,
    JumpGt,
    JumpLt,

    // function management
    Call,
    Return,

    Exit 
}

// execution context
pub struct Context {
    pc: usize,

    stack: Vec<i64>, // LIFO stack here is just a logical concept not rust physical call stack

    call_stack: Vec<usize>,

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
        Context { pc: 0, stack: Vec::new(), call_stack: Vec::new(), registers: [0; 11], memory: HashMap::new(), program}
    }

    // added debug mode
    pub fn run(&mut self, debug: bool) -> Result<i64, String> {
        while self.pc < self.program.len() {
            let instruction = self.program[self.pc].clone();
            
            // Only print debug info if debug is true
            if debug {
                println!("PC: {}, Executing: {:?}", self.pc, instruction);
                println!("Stack before: {:?}", self.stack);
            }
            
            // Execute instruction
            self.execute_ix(instruction)?;
            
            // Only print debug info if debug is true
            if debug {
                println!("Stack after: {:?}", self.stack);
                println!("Registers: {:?}", self.registers);
                println!("-------------------");
            }
            
            if matches!(self.program[self.pc].opcode, OpCode::Exit) {
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
                self.stack.pop().ok_or("Stack Underflow => => b in Pop Op")?;
                self.pc += 1;
            }
            OpCode::Add => {
                let b = self.stack.pop().ok_or("Stack Underflow => b in Add Op")?;
                let a = self.stack.pop().ok_or("Stack Underflow => b in Add Op")?;

                self.stack.push(a + b);

                self.pc += 1;
            },
            OpCode::Sub => {
                let b = self.stack.pop().ok_or("Stack underflow => b in Sub Op")?;
                let a = self.stack.pop().ok_or("Stack underflow => b in Sub Op")?;
                self.stack.push(a - b);
                self.pc += 1;
            },          
            OpCode::Mul => {
                let b = self.stack.pop().ok_or("Stack underflow => b in Mul Op")?;
                let a = self.stack.pop().ok_or("Stack underflow => b in Mul Op")?;
                self.stack.push(a * b);
                self.pc += 1;
            },            
            OpCode::Div => {
                let b = self.stack.pop().ok_or("Stack underflow => b in Div Op")?;
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                let a = self.stack.pop().ok_or("Stack underflow => a in Div op")?;
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
                }
                // fixed unreacheable bug
                let value = self.stack.pop().ok_or("Stack Overflow => StoreReg Op")?;
                self.registers[reg_idx] = value;
                self.pc += 1;
            },
            //control flow
            OpCode::Jump => {
                if instruction.operands.is_empty() {
                    return Err("Jump requires a target address operand".to_string());
                }
                let target = instruction.operands[0] as usize;
                if target >= self.program.len() {
                    return Err(format!("Jump target out of bounds: {}", target));
                }

                self.pc = target;
                return Ok(());
            },
            OpCode::JumpEq => {
                if instruction.operands.is_empty() {
                    return Err("JumpEq requires a target address operand".to_string());
                }
                let target = instruction.operands[0] as usize;
                if target >= self.program.len() {
                    return Err(format!("Jump target out of bounds: {}", target));
                }

                let b = self.stack.pop().ok_or("Stack underflow => b in JumpEq Op")?;
                let a = self.stack.pop().ok_or("Stack underflow => a in JumpEq Op")?;

                if a == b {
                    self.pc = target;
                    return Ok(());
                }

                self.pc += 1;
            },
            OpCode::JumpGt => {
                if instruction.operands.is_empty() {
                    return Err("JumpGt requires a target address operand".to_string());
                }

                let target = instruction.operands[0] as usize;
                if target >= self.program.len() {
                    return Err(format!("Jump target out of bounds: {}", target));
                }

                let b = self.stack.pop().ok_or("Stack underflow => b in JumpGt Op")?;
                let a = self.stack.pop().ok_or("Stack underflow => a in JumpGt Op")?;

                if a > b {
                    self.pc = target;
                    return Ok(());
                }

                self.pc += 1;
            },
            OpCode::JumpLt => {
                if instruction.operands.is_empty() {
                    return Err("JumpGt requires a target address operand".to_string());
                }

                let target = instruction.operands[0] as usize;
                if target >= self.program.len() {
                    return Err(format!("Jump target out of bounds: {}", target));
                }

                let b = self.stack.pop().ok_or("Stack underflow => b in JumpLt Op")?;
                let a = self.stack.pop().ok_or("Stack underflow => b in JumpLt Op")?;

                if a < b {
                    self.pc = target;
                    return Ok(());
                }

                self.pc += 1;
            },
            // fn management
            OpCode::Call => {
                if instruction.operands.is_empty() {
                    return Err("Call requires a function address operand".to_string());
                }
                let func_addr = instruction.operands[0] as usize;
                if func_addr > self.program.len() {
                    return Err(format!("Function address out of bounds: {}", func_addr));
                }
                // save return address -> next ix after call
                self.call_stack.push(self.pc + 1);

                //Jump to fn
                self.pc = func_addr;
                return Ok(());
            },
            OpCode::Return => {
                let return_addr = self.call_stack.pop().ok_or("Call stack underflow (unmatched return)")?;
                self.pc = return_addr;

                return Ok(());
            },
            // mem ops
            OpCode::Load => {
                if instruction.operands.is_empty() {
                    return Err("Load requires an address operand".to_string());
                }
                let addr = instruction.operands[0] as usize;
                let value = *self.memory.get(&addr).unwrap_or(&0);
                self.stack.push(value);

                self.pc += 1;
            },
            OpCode::Store => {
                if instruction.operands.is_empty() {
                    return Err("Store requires an address operand".to_string());
                }
                let addr = instruction.operands[0] as usize;

                let value = self.stack.pop().ok_or("Stack Underflow => value in Store Op")?;
                self.memory.insert(addr, value);

                self.pc += 1;
            },
            OpCode::Exit => {
                return Ok(());
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    // Example program: Calculate factorial of 5
    let program = vec![
        // Initialize r1 with input value (5)
        Instruction { opcode: OpCode::Push, operands: vec![5] },
        Instruction { opcode: OpCode::StoreReg, operands: vec![1] },
        
        // Initialize r0 with accumulator (1)
        Instruction { opcode: OpCode::Push, operands: vec![1] },
        Instruction { opcode: OpCode::StoreReg, operands: vec![0] },
        
        // Loop start at position 4
        // Check if r1 <= 1
        Instruction { opcode: OpCode::LoadReg, operands: vec![1] },
        Instruction { opcode: OpCode::Push, operands: vec![1] },
        Instruction { opcode: OpCode::JumpEq, operands: vec![16] },  // Jump to exit if r1 == 1, this should be 16, not 12!
        
        // Multiply: r0 = r0 * r1
        Instruction { opcode: OpCode::LoadReg, operands: vec![0] },
        Instruction { opcode: OpCode::LoadReg, operands: vec![1] },
        Instruction { opcode: OpCode::Mul, operands: vec![] },
        Instruction { opcode: OpCode::StoreReg, operands: vec![0] },
        
        // Decrement r1
        Instruction { opcode: OpCode::LoadReg, operands: vec![1] },
        Instruction { opcode: OpCode::Push, operands: vec![1] },
        Instruction { opcode: OpCode::Sub, operands: vec![] },
        Instruction { opcode: OpCode::StoreReg, operands: vec![1] },
        
        // Jump back to loop start
        Instruction { opcode: OpCode::Jump, operands: vec![4] },
        
        // Exit program (result in r0)
        Instruction { opcode: OpCode::Exit, operands: vec![] },
    ];

    let mut context = Context::new(program);
    let result = context.run(true)?;
    
    println!("Result: {}", result);  // Should print 120 (5!)
    
    Ok(())
}