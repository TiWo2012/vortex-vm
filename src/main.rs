use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Null,
    Push(i32),
    Pop,
    Ret,
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
            _ => {
                eprintln!("Unknown instruction: {}", line);
            }
        }
    }

    result
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
}
