use crate::instruction::Instruction;

pub fn split_instructions(instructions: &String) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in instructions.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') {
            continue;
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
            "ADDS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::AddS(val));
                    }
                }
            }
            "ADD" => result.push(Instruction::Add),
            "SUBS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::SubS(val));
                    }
                }
            }
            "SUB" => result.push(Instruction::Sub),
            "DUP" => result.push(Instruction::Dup),
            "SWAP" => result.push(Instruction::Swap),
            "MULTS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::MultS(val));
                    }
                }
            }
            "MULT" => result.push(Instruction::Mult),
            "DIVS" => {
                if parts.len() == 2 {
                    if let Ok(val) = parts[1].parse::<i32>() {
                        result.push(Instruction::DivS(val));
                    }
                }
            }
            "DIV" => result.push(Instruction::Div),
            "MEMWRITE" => {
                if parts.len() >= 2 {
                    if let Ok(addr) = parts[1].parse::<i32>() {
                        let values: Vec<i32> = parts[2..]
                            .iter()
                            .filter_map(|v| v.parse::<i32>().ok())
                            .collect();
                        result.push(Instruction::MemWrite(addr, values));
                    }
                }
            }
            "PRINT" => {
                if parts.len() == 3 {
                    if let Ok(addr) = parts[1].parse::<i32>() {
                        if let Ok(len) = parts[2].parse::<i32>() {
                            result.push(Instruction::Print(addr, len));
                        }
                    }
                }
            }
            "MEMREAD" => {
                if parts.len() == 2 {
                    if let Ok(index) = parts[1].parse::<i32>() {
                        result.push(Instruction::MemRead(index));
                    }
                }
            }
            _ => eprintln!("Unknown instruction: {}", line),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_memwrite_parse() {
        let input = "MemWrite 10 1 2 3 4".to_string();
        let parsed = split_instructions(&input);
        assert_eq!(parsed, vec![Instruction::MemWrite(10, vec![1, 2, 3, 4])]);
    }

    #[test]
    fn test_print_parse() {
        let input = "Print 5 3".to_string();
        let parsed = split_instructions(&input);
        assert_eq!(parsed, vec![Instruction::Print(5, 3)]);
    }

    #[test]
    fn test_push_and_pop() {
        let input = "PUSH 42\nPOP\n".to_string();
        let parsed = split_instructions(&input);
        assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Pop]);
    }

    #[test]
    fn test_memwrite() {
        let input = "memwrite 0 1 2\n memread 1".to_string();
        let parsed = split_instructions(&input);
        assert_eq!(
            parsed,
            vec![
                Instruction::MemWrite(0, vec![1, 2]),
                Instruction::MemRead(1)
            ]
        );
    }
}
