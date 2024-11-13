use super::{parser::AST, Instruction};
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum CodegenError {
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodegenError::FailStar => write!(f, "Failed to generate code for star"),
            CodegenError::FailOr => write!(f, "Failed to generate code for or"),
            CodegenError::FailQuestion => write!(f, "Failed to generate code for question"),
        }
    }
}

impl Error for CodegenError {}

#[derive(Debug, Default)]
struct Generator {
    pc: usize,
    instructions: Vec<Instruction>,
}

impl Generator {
    fn inc_pc(&mut self) -> () {
        self.pc += 1;
    }

    fn gen_code(&mut self, ast: &AST) -> Result<(), CodegenError> {
        self.gen_expr(ast)?;
        self.inc_pc();
        self.instructions.push(Instruction::Match);
        Ok(())
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodegenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c),
            AST::Plus(ast) => self.gen_plus(ast),
            AST::Star(ast) => self.gen_star(ast),
            AST::Question(ast) => self.gen_question(ast),
            AST::Or(left_ast, right_ast) => self.gen_or(left_ast, right_ast),
            AST::Seq(exprs) => self.gen_seq(exprs),
        }
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodegenError> {
        self.instructions.push(Instruction::Char(c));
        self.inc_pc();
        Ok(())
    }

    fn gen_plus(&mut self, ast: &AST) -> Result<(), CodegenError> {
        let l1_pc = self.pc;
        self.gen_expr(ast)?;
        self.inc_pc();
        let l2_pc = self.pc;
        let split = Instruction::Split(l1_pc, l2_pc);
        self.instructions.push(split);

        Ok(())
    }

    fn gen_star(&mut self, ast: &AST) -> Result<(), CodegenError> {
        let l1_pc = self.pc;
        self.inc_pc();
        let l2_pc = self.pc;
        let split = Instruction::Split(l2_pc, 0);
        self.instructions.push(split);
        self.gen_expr(ast)?;
        let jump = Instruction::Jump(l1_pc);
        self.instructions.push(jump);
        self.inc_pc();
        let l3_pc = self.pc;

        if let Some(Instruction::Split(_, target)) = self.instructions.get_mut(l1_pc) {
            *target = l3_pc;
        } else {
            return Err(CodegenError::FailStar);
        }

        Ok(())
    }

    fn gen_question(&mut self, ast: &AST) -> Result<(), CodegenError> {
        let split_pc = self.pc;
        self.inc_pc();
        let l1_pc = self.pc;
        let split = Instruction::Split(l1_pc, 0);
        self.instructions.push(split);
        self.gen_expr(ast)?;
        let l2_pc = self.pc;

        if let Some(Instruction::Split(_, target)) = self.instructions.get_mut(split_pc) {
            *target = l2_pc;
        } else {
            return Err(CodegenError::FailQuestion);
        }

        Ok(())
    }

    fn gen_or(&mut self, left_ast: &AST, right_ast: &AST) -> Result<(), CodegenError> {
        let split_pc = self.pc;
        self.inc_pc();
        let l1_pc = self.pc;
        let split = Instruction::Split(l1_pc, 0);
        self.instructions.push(split);
        self.gen_expr(left_ast)?;
        let jump_pc = self.pc;
        let jump = Instruction::Jump(0);
        self.instructions.push(jump);
        self.inc_pc();
        let l2_pc = self.pc;
        self.gen_expr(right_ast)?;
        let l3_pc = self.pc;

        if let Some(Instruction::Split(_, target)) = self.instructions.get_mut(split_pc) {
            *target = l2_pc;
        } else {
            return Err(CodegenError::FailOr);
        }

        if let Some(Instruction::Jump(target)) = self.instructions.get_mut(jump_pc) {
            *target = l3_pc;
        } else {
            return Err(CodegenError::FailOr);
        }

        Ok(())
    }

    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodegenError> {
        for expr in exprs {
            self.gen_expr(expr)?;
        }
        Ok(())
    }
}

pub fn generate_code(ast: &AST) -> Result<Vec<Instruction>, CodegenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.instructions)
}
