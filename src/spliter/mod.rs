use crate::Instruction;

pub fn split_instructions(instructions: &String) -> Vec<Instruction> {
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
            "ADD" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::Add(val));
                    }
                }
            }
            "SUB" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::Sub(val));
                    }
                }
            }
            "DUP" => {
                result.push(Instruction::Dup);
            }
            "SWAP" => {
                result.push(Instruction::Swap);
            }
            "MULTS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::MultS(val));
                    }
                }
            }
            "MULT" => {
                result.push(Instruction::Mult);
            }
            "DIVS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::DivS(val));
                    }
                }
            }
            "DIV" => {
                result.push(Instruction::Div);
            }
            _ => {
                eprintln!("Unknown instruction: {}", line);
            }
        }
    }

    result
}
