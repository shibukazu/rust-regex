use super::Instruction;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    InvalidPC,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::InvalidPC => write!(f, "Invalid program counter"),
        }
    }
}

impl Error for EvalError {}

fn _eval(
    instructions: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    // 正規表現から生成された命令列をMatchまで走り切れればマッチ
    loop {
        let next = if let Some(i) = instructions.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next {
            Instruction::Char(c) => {
                if let Some(line_c) = line.get(sp) {
                    if c == line_c {
                        pc += 1;
                        sp += 1;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Split(a, b) => {
                if _eval(instructions, line, *a, sp)? || _eval(instructions, line, *b, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Match => {
                return Ok(true);
            }
        }
    }
}

pub fn eval(instructions: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    _eval(instructions, line, 0, 0)
}
