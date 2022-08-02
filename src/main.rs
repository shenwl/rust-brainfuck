mod opcode;

use std::io::{Write, Read};

use opcode::{OpCode, Code};

struct Interpreter {
    stack: Vec<u8>, // 表示图灵机的纸带
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0; 1] }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();

        let mut pc = 0; // pc 指针
        let mut sp = 0; // 栈指针

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                OpCode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0);
                    }
                },                  
                OpCode::SHL => {
                    // sp 为 0,不能往左边走了
                    if sp != 0 {
                        sp -= 1;
                    }
                },                  
                OpCode::ADD => {
                    // u8 溢出处理
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                },                  
                OpCode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;

                },                  
                OpCode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                },                  
                OpCode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                },                  
                OpCode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                },                  
                OpCode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                },
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let data = std::fs::read(&args[1])?;

    let mut Interpreter = Interpreter::new();
    Interpreter.run(data)?;

    Ok(())
}
