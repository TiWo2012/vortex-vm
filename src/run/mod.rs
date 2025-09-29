use crate::Instruction;

pub fn run(instructions: &[Instruction]) {
    let mut stack: Vec<i32> = Vec::new();
    let mut i: usize = 0;

    while i < instructions.len() {
        match &instructions[i] {
            Instruction::Null => {
                i += 1;
            }
            Instruction::Push(value) => {
                stack.push(*value);
                println!("PUSH {} => {:?}", value, stack);
                i += 1;
            }
            Instruction::Pop => {
                if let Some(val) = stack.pop() {
                    println!("POP {} => {:?}", val, stack);
                } else {
                    eprintln!("Stack underflow!");
                }
                i += 1;
            }
            Instruction::Ret => {
                if let Some(val) = stack.pop() {
                    println!("Program returned with exit code {}", val);
                } else {
                    eprintln!("No value on stack to return");
                }
                break;
            }
            Instruction::Jiz(target) => {
                if let Some(&val) = stack.last() {
                    if val == 0 {
                        if *target >= 0 && (*target as usize) < instructions.len() {
                            i = *target as usize;
                            println!("JIZ taken to {}", i);
                            continue; // don’t do i += 1
                        } else {
                            eprintln!("Invalid jump target {}", target);
                            break;
                        }
                    }
                } else {
                    eprintln!("No value on stack to compare (JIZ)");
                }
                i += 1;
            }
            Instruction::Jnz(target) => {
                if let Some(&val) = stack.last() {
                    if val != 0 {
                        i = *target as usize;
                        continue;
                    }
                } else {
                    eprintln!("No value on stack to compare (JNZ)");
                }
                i += 1;
            }
            Instruction::Add(target) => {
                if let Some(&val) = stack.last() {
                    if val != 0 {
                        stack.push(val + target);
                    }
                } else {
                    eprintln!("No value on stack to add (JNZ)");
                }
                i += 1;
            }
            Instruction::Sub(target) => {
                if let Some(&val) = stack.last() {
                    if val != 0 {
                        stack.push(val - target);
                    }
                } else {
                    eprintln!("No value on stack to add (JNZ)");
                }
                i += 1;
            }
            Instruction::Dup => {
                if let Some(&val) = stack.last() {
                    stack.push(val);
                } else {
                    eprintln!("No value on stack to add (JNZ)");
                }
                i += 1;
            }
            Instruction::Swap => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                stack.push(a);
                stack.push(b);

                i += 1;
            }
            Instruction::DivS(target) => {
                if let Some(&val) = stack.last() {
                    stack.push(val / target);
                } else {
                    eprintln!("No value on stack to divide (DivS)");
                }
                i += 1;
            }
            Instruction::Div => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = b / a;

                stack.push(b);
                stack.push(a);
                stack.push(res);

                i += 1;
            }
            Instruction::MultS(target) => {
                if let Some(&val) = stack.last() {
                    stack.push(val * target);
                } else {
                    eprintln!("No value on stack to divide (DivS)");
                }
                i += 1;
            }
            Instruction::Mult => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = b * a;

                stack.push(b);
                stack.push(a);
                stack.push(res);

                i += 1;
            }
        }
    }

    println!("Final stack: {:?}", stack);
}
