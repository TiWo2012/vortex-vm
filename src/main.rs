use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Null,
    Push(i32),
    Pop,
    Ret,
    Jiz(i32),
    Jnz(i32),
}

fn split_instructions(instructions: &String) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in instructions.lines() {
        // Trim spaces + Windows `\r`
        let line = line.trim();

        if line.is_empty() {
            continue; // skip blank lines
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0].to_uppercase().as_str() {
            "NULL" => result.push(Instruction::Null),
            "PUSH" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::Push(val));
                    }
                }
            }
            "POP" => result.push(Instruction::Pop),
            "RET" => result.push(Instruction::Ret),
            "JIZ" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::Jiz(val));
                    }
                }
            }
            "JNZ" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::Jnz(val));
                    }
                }
            }
            _ => {
                eprintln!("Unknown instruction: {}", line);
            }
        }
    }

    result
}

fn run(instructions: &[Instruction]) {
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
        }
    }

    println!("Final stack: {:?}", stack);
}

fn main() {
    // step 0: get command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: vm <filename>");
        return;
    }

    // step 1: read the file
    let filename = &args[1];
    let instructions = fs::read_to_string(filename).expect("Failed to read file");

    // step 2: split the file into tokens
    let instruction_arr: Vec<Instruction> = split_instructions(&instructions);
    println!("{:?}", instruction_arr);

    // step 3: run the instructions
    run(&instruction_arr);
}
