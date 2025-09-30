#[derive(Debug, PartialEq)]
pub enum Instruction {
    Null,
    Push(i32),
    Pop,
    Ret,
    Jiz(i32),
    Jnz(i32),
    AddS(i32),
    Add,
    SubS(i32),
    Sub,
    MultS(i32),
    Mult,
    DivS(i32),
    Div,
    Dup,
    Swap,
}
