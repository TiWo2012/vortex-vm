use crate::instruction::Instruction;

pub fn split_instructions(instructions: &String) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in instructions.lines() {
        // Trim spaces + Windows `\r`
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_push_and_pop() {
        let input = "PUSH 42\nPOP\n".to_string();
        let parsed = split_instructions(&input);

        assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Pop,]);
    }

    #[test]
    fn test_arithmetic() {
        let input = "PUSH 10\nPUSH 5\nADD 3\nSUB 2\nMULT\nDIV\n".to_string();
        let parsed = split_instructions(&input);

        assert_eq!(
            parsed,
            vec![
                Instruction::Push(10),
                Instruction::Push(5),
                Instruction::Add(3),
                Instruction::Sub(2),
                Instruction::Mult,
                Instruction::Div,
            ]
        );
    }

    #[test]
    fn test_jumps_and_ret() {
        let input = "JIZ 4\nJNZ 7\nRET\n".to_string();
        let parsed = split_instructions(&input);

        assert_eq!(
            parsed,
            vec![Instruction::Jiz(4), Instruction::Jnz(7), Instruction::Ret,]
        );
    }

    #[test]
    fn test_duplicate_and_swap() {
        let input = "DUP\nSWAP\n".to_string();
        let parsed = split_instructions(&input);

        assert_eq!(parsed, vec![Instruction::Dup, Instruction::Swap,]);
    }

    #[test]
    fn test_invalid_instruction() {
        let input = "FOO\n".to_string();
        let parsed = split_instructions(&input);

        assert!(parsed.is_empty());
    }
}
