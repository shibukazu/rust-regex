mod codegen;
mod evaluator;
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

pub fn do_matching(expr: &str, line: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let ast = parser::parse(expr)?;
    let instructions = codegen::generate_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    println!(
        "line: {:?}, expr: {:?}, instructions: {:?}",
        line, expr, instructions
    );
    Ok(evaluator::eval(&instructions, &line)?)
}
