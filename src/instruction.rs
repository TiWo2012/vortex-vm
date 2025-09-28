#[derive(Debug)]
pub enum Instruction {
    Null,
    Push(i32),
    Pop,
    Ret,
    Jiz(i32),
    Jnz(i32),
}
