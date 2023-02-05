#[derive(Debug)]
pub enum Instruction{
    Char(char),
    Match,
    Jump(usize),
    Split(usize,usize),
}

impl Display for Instruction{
    fn fmt(&self,f: &mut fnt::Formatter<'_>) -> fmt::Result{
        match self{
            Instruction::Char => write!(f,"char {}",c),
            Instruction::Match => write!(f,"match"),
            Instruction::Jump(addr) => write!(f,"Jump {:>04}",addr),
            Instruction::Split(addr1,addr2) => write!(f,"split {:>04}, {:>04}",addr1,addr2),
        }
    }
}