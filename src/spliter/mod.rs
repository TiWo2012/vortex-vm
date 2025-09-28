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
            _ => {
                eprintln!("Unknown instruction: {}", line);
            }
        }
    }

    result
}
