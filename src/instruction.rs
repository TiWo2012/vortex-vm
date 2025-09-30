#[derive(Debug, PartialEq)]
pub enum Instruction {
    Null,

    Push(i32),
    Dup,
    Swap,
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

    //       start, length, values
    MemWrite(i32, Vec<i32>),
    Print(i32, i32),
}
