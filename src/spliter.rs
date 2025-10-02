use crate::instruction::Instruction;
use std::collections::HashMap;

/// Parses assembly code into a sequence of instructions with label resolution.
///
/// Uses a two-pass algorithm:
/// 1. First pass: Collect all label definitions and their instruction positions
/// 2. Second pass: Parse instructions and resolve label references to addresses
///
/// # Examples
///
/// Basic instruction parsing:
///
/// ```
/// use vortex_vm::spliter::split_instructions;
/// use vortex_vm::instruction::Instruction;
///
/// let assembly = "PUSH 42\nADD\nRET";
/// let instructions = split_instructions(assembly);
///
/// assert_eq!(instructions, vec![
///     Instruction::Push(42),
///     Instruction::Add,
///     Instruction::Ret,
/// ]);
/// ```
///
/// Label resolution:
///
/// ```
/// use vortex_vm::spliter::split_instructions;
/// use vortex_vm::instruction::Instruction;
///
/// let assembly = "
///     main:
///     PUSH 10
///     SUBS 1
///     JNZ main
///     RET
/// ";
/// let instructions = split_instructions(assembly);
///
/// // The label "main" should be resolved to address "0"
/// if let Instruction::Jnz(target) = &instructions[2] {
///     assert_eq!(target, "0");
/// }
/// ```
///
/// Memory operations with comments:
///
/// ```
/// use vortex_vm::spliter::split_instructions;
/// use vortex_vm::instruction::Instruction;
///
/// let assembly = "
///     ; Write hello to memory
///     MemWrite 0 72 101 108 108 111
///     ; Print the message
///     Print 0 5
///     RET
/// ";
/// let instructions = split_instructions(assembly);
///
/// assert_eq!(instructions, vec![
///     Instruction::MemWrite(0, vec![72, 101, 108, 108, 111]),
///     Instruction::Print(0, 5),
///     Instruction::Ret,
/// ]);
/// ```
pub fn split_instructions(instructions: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut labels = HashMap::new();

    // Phase 1: Collect all labels and map them to instruction indices
    collect_labels(instructions, &mut labels);

    // Phase 2: Parse instructions and resolve label references
    parse_instructions(instructions, &labels, &mut result);

    // Phase 3: Replace label references with actual instruction indices
    resolve_label_references(&mut result, &labels);

    result
}

/// First pass: Scan through all lines to find label definitions and record their positions.
/// Labels are identified by lines ending with ':' (after removing comments and whitespace).
fn collect_labels(instructions: &str, labels: &mut HashMap<String, usize>) {
    let mut instruction_index = 0;

    for line in instructions.lines() {
        let clean_line = extract_code_portion(line);

        if clean_line.is_empty() || is_comment_line(clean_line) {
            continue;
        }

        if is_label_definition(clean_line) {
            let label_name = extract_label_name(clean_line);
            labels.insert(label_name, instruction_index);
        } else {
            // This is an instruction, so it takes up an instruction slot
            instruction_index += 1;
        }
    }
}

/// Second pass: Parse each line as an instruction, ignoring labels and comments.
/// Label references (like "main" or "loop") are kept as strings for later resolution.
fn parse_instructions(instructions: &str, _labels: &HashMap<String, usize>, result: &mut Vec<Instruction>) {
    for line in instructions.lines() {
        let clean_line = extract_code_portion(line);

        if clean_line.is_empty() || is_comment_line(clean_line) || is_label_definition(clean_line) {
            continue;
        }

        if let Some(instruction) = parse_instruction_line(clean_line) {
            result.push(instruction);
        }
    }
}

/// Third pass: Replace all label references in jump instructions with their actual instruction indices.
/// Converts labels like "main" to their corresponding instruction index as a string.
fn resolve_label_references(instructions: &mut [Instruction], labels: &HashMap<String, usize>) {
    for instruction in instructions.iter_mut() {
        match instruction {
            Instruction::Jiz(target) | Instruction::Jnz(target) => {
                if let Some(&address) = labels.get(target) {
                    // Replace label with its instruction index
                    *target = address.to_string();
                } else if target.parse::<usize>().is_ok() {
                    // It's already a numeric address, keep it as string
                    // No change needed
                } else {
                    eprintln!("Warning: Unknown label or invalid address: {}", target);
                }
            }
            _ => {
                // Not a jump instruction, no label resolution needed
            }
        }
    }
}

/// Extracts the code portion of a line, removing comments and whitespace.
/// Everything after the first ';' is considered a comment and ignored.
fn extract_code_portion(line: &str) -> &str {
    let trimmed = line.trim();

    if let Some(semicolon_pos) = trimmed.find(';') {
        trimmed[..semicolon_pos].trim()
    } else {
        trimmed
    }
}

/// Checks if a line is a comment (either starts with ';' or is empty after comment removal).
fn is_comment_line(line: &str) -> bool {
    line.starts_with(';') || line.is_empty()
}

/// Checks if a line is a label definition (ends with ':').
fn is_label_definition(line: &str) -> bool {
    line.ends_with(':')
}

/// Extracts the label name from a label definition line (removes the ':' suffix).
fn extract_label_name(line: &str) -> String {
    line.strip_suffix(':').unwrap_or(line).trim().to_string()
}

/// Parses a single instruction line into an Instruction enum variant.
/// Handles all supported instruction types with their parameters.
fn parse_instruction_line(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    match parts[0].to_uppercase().as_str() {
        // Basic stack operations
        "NULL" => Some(Instruction::Null),
        "PUSH" => parse_push_instruction(&parts),
        "POP" => Some(Instruction::Pop),
        "DUP" => Some(Instruction::Dup),
        "SWAP" => Some(Instruction::Swap),

        // Control flow
        "RET" => Some(Instruction::Ret),
        "JIZ" => parse_jump_instruction(&parts, Instruction::Jiz),
        "JNZ" => parse_jump_instruction(&parts, Instruction::Jnz),

        // Arithmetic operations
        "ADD" => Some(Instruction::Add),
        "ADDS" => parse_arithmetic_immediate(&parts, Instruction::AddS),
        "SUB" => Some(Instruction::Sub),
        "SUBS" => parse_arithmetic_immediate(&parts, Instruction::SubS),
        "MULT" => Some(Instruction::Mult),
        "MULTS" => parse_arithmetic_immediate(&parts, Instruction::MultS),
        "DIV" => Some(Instruction::Div),
        "DIVS" => parse_arithmetic_immediate(&parts, Instruction::DivS),

        // Memory operations
        "MEMWRITE" => parse_memwrite_instruction(&parts),
        "MEMWRITES" => parse_memwrites_instruction(&parts),
        "MEMREAD" => parse_memread_instruction(&parts),
        "PRINT" => parse_print_instruction(&parts),

        // Unknown instruction
        _ => {
            eprintln!("Unknown instruction: {}", line);
            None
        }
    }
}

/// Parses a PUSH instruction with its integer value parameter.
fn parse_push_instruction(parts: &[&str]) -> Option<Instruction> {
    if parts.len() == 2 {
        parts[1].parse::<i32>().ok().map(Instruction::Push)
    } else {
        None
    }
}

/// Parses jump instructions (JIZ, JNZ) with their target address/label parameter.
fn parse_jump_instruction<F>(parts: &[&str], constructor: F) -> Option<Instruction>
where
    F: FnOnce(String) -> Instruction,
{
    if parts.len() == 2 {
        Some(constructor(parts[1].to_string()))
    } else {
        None
    }
}

/// Parses arithmetic immediate instructions (ADDS, SUBS, MULTS, DIVS) with their integer parameter.
fn parse_arithmetic_immediate<F>(parts: &[&str], constructor: F) -> Option<Instruction>
where
    F: FnOnce(i32) -> Instruction,
{
    if parts.len() == 2 {
        parts[1].parse::<i32>().ok().map(constructor)
    } else {
        None
    }
}

/// Parses the MEMWRITE instruction with address and multiple values.
fn parse_memwrite_instruction(parts: &[&str]) -> Option<Instruction> {
    if parts.len() >= 2 {
        if let Ok(addr) = parts[1].parse::<i32>() {
            let values: Vec<i32> = parts[2..]
                .iter()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect();
            Some(Instruction::MemWrite(addr, values))
        } else {
            None
        }
    } else {
        None
    }
}

/// Parses the MEMWRITES instruction with address and length parameters.
fn parse_memwrites_instruction(parts: &[&str]) -> Option<Instruction> {
    if parts.len() == 3 {
        if let (Ok(addr), Ok(len)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
            Some(Instruction::MemWriteS(addr, len))
        } else {
            None
        }
    } else {
        None
    }
}

/// Parses the MEMREAD instruction with address parameter.
fn parse_memread_instruction(parts: &[&str]) -> Option<Instruction> {
    if parts.len() == 2 {
        parts[1].parse::<i32>().ok().map(Instruction::MemRead)
    } else {
        None
    }
}

/// Parses the PRINT instruction with address and length parameters.
fn parse_print_instruction(parts: &[&str]) -> Option<Instruction> {
    if parts.len() == 3 {
        if let (Ok(addr), Ok(len)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
            Some(Instruction::Print(addr, len))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    mod stack_operations {
        use super::*;

        #[test]
        fn test_null_parse() {
            let input = "NULL".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Null]);
        }

        #[test]
        fn test_push_parse() {
            let input = "PUSH 42".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Push(42)]);
        }

        #[test]
        fn test_pop_parse() {
            let input = "POP".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Pop]);
        }

        #[test]
        fn test_dup_parse() {
            let input = "DUP".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Dup]);
        }

        #[test]
        fn test_swap_parse() {
            let input = "SWAP".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Swap]);
        }

        #[test]
        fn test_push_and_pop() {
            let input = "PUSH 42\nPOP".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Pop]);
        }
    }

    mod control_flow {
        use super::*;

        #[test]
        fn test_ret_parse() {
            let input = "RET".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Ret]);
        }

        #[test]
        fn test_jiz_parse() {
            let input = "JIZ 5".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Jiz("5".to_string())]);
        }

        #[test]
        fn test_jnz_parse() {
            let input = "JNZ main".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Jnz("main".to_string())]);
        }

        #[test]
        fn test_jumps_with_labels() {
            let input = "JIZ start\nJNZ end".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![
                Instruction::Jiz("start".to_string()),
                Instruction::Jnz("end".to_string())
            ]);
        }
    }

    mod arithmetic_operations {
        use super::*;

        #[test]
        fn test_add_parse() {
            let input = "ADD".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Add]);
        }

        #[test]
        fn test_adds_parse() {
            let input = "ADDS 5".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::AddS(5)]);
        }

        #[test]
        fn test_sub_parse() {
            let input = "SUB".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Sub]);
        }

        #[test]
        fn test_subs_parse() {
            let input = "SUBS 3".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::SubS(3)]);
        }

        #[test]
        fn test_mult_parse() {
            let input = "MULT".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Mult]);
        }

        #[test]
        fn test_mults_parse() {
            let input = "MULTS 2".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::MultS(2)]);
        }

        #[test]
        fn test_div_parse() {
            let input = "DIV".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Div]);
        }

        #[test]
        fn test_divs_parse() {
            let input = "DIVS 4".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::DivS(4)]);
        }
    }

    mod memory_operations {
        use super::*;

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
        fn test_memread_parse() {
            let input = "MemRead 5".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::MemRead(5)]);
        }

        #[test]
        fn test_print_parse() {
            let input = "Print 5 3".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Print(5, 3)]);
        }

        #[test]
        fn test_memwrite_complex() {
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

    mod comment_and_edge_cases {
        use super::*;

        #[test]
        fn test_inline_comments() {
            let input = "PUSH 42 ; This is a comment\nPOP ; Another comment".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Pop]);
        }

        #[test]
        fn test_empty_lines() {
            let input = "\nPUSH 42\n\nPOP\n".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Pop]);
        }

        #[test]
        fn test_case_insensitive() {
            let input = "push 42\nADD\nPop".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![Instruction::Push(42), Instruction::Add, Instruction::Pop]);
        }

        #[test]
        fn test_label_parsing() {
            let input = "
                main:
                PUSH 10
                SUBS 1
                JNZ main
                RET
            ".to_string();
            let parsed = split_instructions(&input);

            // The label "main" should be resolved to address "0"
            if let Instruction::Jnz(target) = &parsed[2] {
                assert_eq!(target, "0");
            }
        }

        #[test]
        fn test_multiple_instructions() {
            let input = "PUSH 1\nPUSH 2\nADD\nPUSH 3\nMULT\nRET".to_string();
            let parsed = split_instructions(&input);
            assert_eq!(parsed, vec![
                Instruction::Push(1),
                Instruction::Push(2),
                Instruction::Add,
                Instruction::Push(3),
                Instruction::Mult,
                Instruction::Ret
            ]);
        }
    }
}
