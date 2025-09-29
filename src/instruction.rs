#[derive(Debug)]
pub enum Instruction {
    Null,
    Push(i32),
    Pop,
    Ret,
    Jiz(i32),
    Jnz(i32),
    Add(i32),
    Sub(i32),
    MultS(i32),
    Mult,
    DivS(i32),
    Div,
    Dup,
    Swap,
}
