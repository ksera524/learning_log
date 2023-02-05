use super::{parser::AST ,Instruction};
use crate::helper::safe_add;
use std::{
    error::Error,
    f,t::{self,Display}
};

#[derive(Debug)]
pub enum CodeGenError{
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"CodeGenError: {:?}",self)
    }
}

impl Error for CodeGenError{}

#[derive(Default,Debug)]
struct Generator {
    pc:usize,
    insts: Vec<Instruction>,
}

impl Generator{
    fn inc_pc(&mut self) -> Result<(),CodeGenError> {
        safe_add(&mut self.pc,&1 || CodeGenError::PCOverFlow)
    }

    fn gen_expr(&mut self,ast: &AST) -> Result<(),CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(e1,e2) => self.gen_or(e1,e2)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e) => self.gen_star(e)?,
            AST::Question(e) => self.gen_question(e)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    fn gen_seq(&mut self,exprs:&[AST]) -> Result<(),CodeGenError>{
        for e in exprs{
            self.gen_expr(e)?;
        }
        Ok(())
    }

    fn gen_char(&mut self,c:char) -> Result<(),CodeGenError>{
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_or(&mut self,e1:&AST,e2:&AST) -> Result<(),CodeGenError>{
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc,0);
        self.insts.push(split);

        self.gen_expr(e1)?;
        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0));

        self.inc_pc()?;
        if let Some(Instruction::Split(_,l2)) = self.insts.get_mut(split_addr){
            *l2 = self.pc;
        } else {
            return Err(Box::new(CodeGenError::FailOr));
        }

        self.gen_expr(e2)?;
        if let Some(Instruction::Jump(l3)) = self.inst.get_mut(jmp_addr){
            *l3 = self.pc;
        } else {
            return Err(Box::new(CodeGenError::FailOr));
        }
        Ok(())
    }
}