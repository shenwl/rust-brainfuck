use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    SHR = 0x3E,     // <
    SHL = 0x3C,     // >
    ADD = 0x2B,     // +
    SUB = 0x2D,     // -
    PUTCHAR = 0x2E, // *
    GETCHAR = 0x2C, // .
    LB = 0x5B,      // [
    RB = 0x5D,      // ]
}

impl From<u8> for OpCode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => OpCode::SHR,
            0x3C => OpCode::SHL,
            0x2B => OpCode::ADD,
            0x2D => OpCode::SUB,
            0x2E => OpCode::PUTCHAR,
            0x2C => OpCode::GETCHAR,
            0x5B => OpCode::LB,
            0x5D => OpCode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code {
    pub instrs: Vec<OpCode>,
    pub jtable: HashMap<usize, usize>, // [] 流程控制的跳转关系，提前记录位置
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            OpCode::SHR as u8,
            OpCode::SHL as u8,
            OpCode::ADD as u8,
            OpCode::SUB as u8,
            OpCode::PUTCHAR as u8,
            OpCode::GETCHAR as u8,
            OpCode::LB as u8,
            OpCode::RB as u8,
        ];
        let instrs: Vec<OpCode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| OpCode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();

        for (i, e) in instrs.iter().enumerate() {
            if OpCode::LB == *e {
                jstack.push(i);
            }
            if OpCode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty stack!")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }
        Ok(Code { instrs, jtable })
    }
}
