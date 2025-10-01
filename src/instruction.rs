#[derive(Debug, PartialEq)]
pub enum Instruction {
    Null,

    Push(i32),
    Dup,
    Swap,
    Pop,
    Ret,

    Jiz(String),
    Jnz(String),

    AddS(i32),
    Add,
    SubS(i32),
    Sub,
    MultS(i32),
    Mult,
    DivS(i32),
    Div,

    MemWrite(i32, Vec<i32>),
    MemWriteS(i32, i32),
    MemRead(i32),
    Print(i32, i32),
}
