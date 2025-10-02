use crate::instruction::Instruction;
use std::collections::HashMap;

pub fn split_instructions(instructions: &String) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    // First pass: collect all labels and their positions
    let mut instruction_index = 0;
    for line in instructions.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Check if line has a comment (semicolon)
        let code_part = if let Some(semicolon_pos) = line.find(';') {
            line[..semicolon_pos].trim()
        } else {
            line
        };

        if code_part.is_empty() || code_part.starts_with(';') {
            continue;
        }

        if code_part.ends_with(':') {
            // This is a label definition
            let label_name = code_part.strip_suffix(':').unwrap().trim().to_string();
            labels.insert(label_name, instruction_index);
        } else {
            // This is an instruction, count it
            instruction_index += 1;
        }
    }

    // Second pass: parse instructions and resolve labels
    for line in instructions.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Check if line has a comment (semicolon)
        let code_part = if let Some(semicolon_pos) = line.find(';') {
            line[..semicolon_pos].trim()
        } else {
            line
        };

        if code_part.is_empty() || code_part.starts_with(';') {
            continue;
        }

        if code_part.ends_with(':') {
            // Skip label definitions in second pass
            continue;
        }

        let parts: Vec<&str> = code_part.split_whitespace().collect();

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
                    let target = parts[1].to_string();
                    result.push(Instruction::Jiz(target));
                }
            }
            "JNZ" => {
                if parts.len() == 2 {
                    let target = parts[1].to_string();
                    result.push(Instruction::Jnz(target));
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
            "MEMWRITES" => {
                if parts.len() == 3 {
                    if let Ok(addr) = parts[1].parse::<i32>() {
                        if let Ok(len) = parts[2].parse::<i32>() {
                            result.push(Instruction::MemWriteS(addr, len));
                        }
                    }
                }
            }
            _ => eprintln!("Unknown instruction: {}", code_part),
        }
    }

    // Now resolve all label references to instruction indices
    for instruction in &mut result {
        match instruction {
            Instruction::Jiz(target) | Instruction::Jnz(target) => {
                if let Some(&address) = labels.get(target) {
                    *target = address.to_string();
                } else if let Ok(address) = target.parse::<usize>() {
                    // It's already a numeric address, keep it as string
                    *target = address.to_string();
                } else {
                    eprintln!("Unknown label or invalid address: {}", target);
                }
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_memwrites_parse() {
        let input = "MemWrites 10 4".to_string();
        let parsed = split_instructions(&input);
        assert_eq!(parsed, vec![Instruction::MemWriteS(10, 4)]);
    }

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
    fn test_inline_comments() {
        let input = "PUSH 42 ; This is a comment\nPOP ; Another comment\n".to_string();
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
