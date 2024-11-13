mod codegen;
mod parser;

use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Split(usize, usize),
    Jump(usize),
    Match,
}
impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "Char({})", c),
            Instruction::Split(a, b) => write!(f, "Split({}, {})", a, b),
            Instruction::Jump(a) => write!(f, "Jump({})", a),
            Instruction::Match => write!(f, "Match"),
        }
    }
}
