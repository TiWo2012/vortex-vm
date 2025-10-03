use crate::instruction::Instruction;
use std::fs;
use std::io::Write;

/// Assembles assembly source code into bytecode format
pub fn assemble_source(source: &str) -> Result<Vec<u8>, String> {
    // Parse the assembly source into instructions
    let instructions = crate::spliter::split_instructions(source);

    // Serialize instructions to binary format
    serialize_instructions(&instructions)
}

/// Deserializes bytecode back into instructions
pub fn disassemble_bytecode(bytecode: &[u8]) -> Result<Vec<Instruction>, String> {
    deserialize_instructions(bytecode)
}

/// Assembles a .asv file to a .vvm file
pub fn assemble_file(input_path: &str, output_path: &str) -> Result<(), String> {
    // Read the source file
    let source = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read source file '{}': {}", input_path, e))?;

    // Assemble the source
    let bytecode = assemble_source(&source)?;

    // Write the bytecode to output file
    fs::write(output_path, bytecode)
        .map_err(|e| format!("Failed to write bytecode file '{}': {}", output_path, e))?;

    Ok(())
}

/// Loads instructions from a .vvm bytecode file
pub fn load_bytecode_file(file_path: &str) -> Result<Vec<Instruction>, String> {
    // Read the bytecode file
    let bytecode = fs::read(file_path)
        .map_err(|e| format!("Failed to read bytecode file '{}': {}", file_path, e))?;

    // Deserialize the instructions
    disassemble_bytecode(&bytecode)
}

/// Serializes instructions to binary format
fn serialize_instructions(instructions: &[Instruction]) -> Result<Vec<u8>, String> {
    let mut bytecode = Vec::new();

    for instruction in instructions {
        serialize_instruction(instruction, &mut bytecode)?;
    }

    Ok(bytecode)
}

/// Deserializes instructions from binary format
fn deserialize_instructions(bytecode: &[u8]) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    let mut offset = 0;

    while offset < bytecode.len() {
        let (instruction, consumed) = deserialize_instruction(&bytecode[offset..])?;
        instructions.push(instruction);
        offset += consumed;
    }

    Ok(instructions)
}

/// Serializes a single instruction to binary format
fn serialize_instruction(instruction: &Instruction, output: &mut Vec<u8>) -> Result<(), String> {
    match instruction {
        Instruction::Null => {
            output.write_all(&[0x00]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Push(value) => {
            output.write_all(&[0x01]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Dup => {
            output.write_all(&[0x02]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Swap => {
            output.write_all(&[0x03]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Pop => {
            output.write_all(&[0x04]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Ret => {
            output.write_all(&[0x05]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Jiz(target) => {
            output.write_all(&[0x06]).map_err(|e| format!("Write error: {}", e))?;
            serialize_string(target, output)?;
        }
        Instruction::Jnz(target) => {
            output.write_all(&[0x07]).map_err(|e| format!("Write error: {}", e))?;
            serialize_string(target, output)?;
        }
        Instruction::AddS(value) => {
            output.write_all(&[0x08]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Add => {
            output.write_all(&[0x09]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::SubS(value) => {
            output.write_all(&[0x0A]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Sub => {
            output.write_all(&[0x0B]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::MultS(value) => {
            output.write_all(&[0x0C]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Mult => {
            output.write_all(&[0x0D]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::DivS(value) => {
            output.write_all(&[0x0E]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Div => {
            output.write_all(&[0x0F]).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::MemWrite(addr, values) => {
            output.write_all(&[0x10]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&addr.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
            let len = values.len() as u32;
            output.write_all(&len.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
            for value in values {
                output.write_all(&value.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
            }
        }
        Instruction::MemWriteS(addr, len) => {
            output.write_all(&[0x11]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&addr.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&len.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::MemRead(addr) => {
            output.write_all(&[0x12]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&addr.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
        Instruction::Print(addr, len) => {
            output.write_all(&[0x13]).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&addr.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
            output.write_all(&len.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        }
    }

    Ok(())
}

/// Deserializes a single instruction from binary format
fn deserialize_instruction(bytes: &[u8]) -> Result<(Instruction, usize), String> {
    if bytes.is_empty() {
        return Err("Empty bytecode".to_string());
    }

    let opcode = bytes[0];
    let mut offset = 1;

    match opcode {
        0x00 => Ok((Instruction::Null, offset)),
        0x01 => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete Push instruction".to_string());
            }
            let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::Push(value), offset))
        }
        0x02 => Ok((Instruction::Dup, offset)),
        0x03 => Ok((Instruction::Swap, offset)),
        0x04 => Ok((Instruction::Pop, offset)),
        0x05 => Ok((Instruction::Ret, offset)),
        0x06 => {
            let (target, consumed) = deserialize_string(&bytes[offset..])?;
            offset += consumed;
            Ok((Instruction::Jiz(target), offset))
        }
        0x07 => {
            let (target, consumed) = deserialize_string(&bytes[offset..])?;
            offset += consumed;
            Ok((Instruction::Jnz(target), offset))
        }
        0x08 => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete AddS instruction".to_string());
            }
            let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::AddS(value), offset))
        }
        0x09 => Ok((Instruction::Add, offset)),
        0x0A => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete SubS instruction".to_string());
            }
            let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::SubS(value), offset))
        }
        0x0B => Ok((Instruction::Sub, offset)),
        0x0C => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete MultS instruction".to_string());
            }
            let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::MultS(value), offset))
        }
        0x0D => Ok((Instruction::Mult, offset)),
        0x0E => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete DivS instruction".to_string());
            }
            let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::DivS(value), offset))
        }
        0x0F => Ok((Instruction::Div, offset)),
        0x10 => {
            if bytes.len() < offset + 12 {
                return Err("Incomplete MemWrite instruction".to_string());
            }
            let addr = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            let len = u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            let mut values = Vec::new();
            for _ in 0..len {
                if bytes.len() < offset + 4 {
                    return Err("Incomplete MemWrite values".to_string());
                }
                let value = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
                values.push(value);
                offset += 4;
            }
            Ok((Instruction::MemWrite(addr, values), offset))
        }
        0x11 => {
            if bytes.len() < offset + 8 {
                return Err("Incomplete MemWriteS instruction".to_string());
            }
            let addr = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            let len = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::MemWriteS(addr, len), offset))
        }
        0x12 => {
            if bytes.len() < offset + 4 {
                return Err("Incomplete MemRead instruction".to_string());
            }
            let addr = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::MemRead(addr), offset))
        }
        0x13 => {
            if bytes.len() < offset + 8 {
                return Err("Incomplete Print instruction".to_string());
            }
            let addr = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            let len = i32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;
            Ok((Instruction::Print(addr, len), offset))
        }
        _ => Err(format!("Unknown opcode: 0x{:02X}", opcode))
    }
}

/// Serializes a string to binary format (null-terminated)
fn serialize_string(s: &str, output: &mut Vec<u8>) -> Result<(), String> {
    output.write_all(s.as_bytes()).map_err(|e| format!("Write error: {}", e))?;
    output.write_all(&[0]).map_err(|e| format!("Write error: {}", e))?; // Null terminator
    Ok(())
}

/// Deserializes a string from binary format (null-terminated)
fn deserialize_string(bytes: &[u8]) -> Result<(String, usize), String> {
    let mut end = 0;
    while end < bytes.len() && bytes[end] != 0 {
        end += 1;
    }

    if end >= bytes.len() {
        return Err("Unterminated string in bytecode".to_string());
    }

    let s = String::from_utf8(bytes[..end].to_vec())
        .map_err(|e| format!("Invalid UTF-8 string in bytecode: {}", e))?;

    Ok((s, end + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_assemble_basic_instructions() {
        let source = "PUSH 42\nADD\nRET";
        let bytecode = assemble_source(source).unwrap();
        let instructions = disassemble_bytecode(&bytecode).unwrap();

        assert_eq!(instructions, vec![
            Instruction::Push(42),
            Instruction::Add,
            Instruction::Ret,
        ]);
    }

    #[test]
    fn test_assemble_jump_instructions() {
        let source = "JIZ main\nJNZ end";
        let bytecode = assemble_source(source).unwrap();
        let instructions = disassemble_bytecode(&bytecode).unwrap();

        assert_eq!(instructions, vec![
            Instruction::Jiz("main".to_string()),
            Instruction::Jnz("end".to_string()),
        ]);
    }

    #[test]
    fn test_assemble_memory_instructions() {
        let source = "MemWrite 0 72 101 108 108 111\nPrint 0 5";
        let bytecode = assemble_source(source).unwrap();
        let instructions = disassemble_bytecode(&bytecode).unwrap();

        assert_eq!(instructions, vec![
            Instruction::MemWrite(0, vec![72, 101, 108, 108, 111]),
            Instruction::Print(0, 5),
        ]);
    }

    #[test]
    fn test_assemble_with_labels() {
        let source = "
            main:
            PUSH 10
            SUBS 1
            JNZ main
            RET
        ";
        let bytecode = assemble_source(source).unwrap();
        let instructions = disassemble_bytecode(&bytecode).unwrap();

        // Label "main" should be resolved to address "0"
        assert_eq!(instructions, vec![
            Instruction::Push(10),
            Instruction::SubS(1),
            Instruction::Jnz("0".to_string()),
            Instruction::Ret,
        ]);
    }

    #[test]
    fn test_round_trip() {
        let original_instructions = vec![
            Instruction::Push(123),
            Instruction::Dup,
            Instruction::Add,
            Instruction::MemWrite(0, vec![1, 2, 3]),
            Instruction::Print(0, 3),
            Instruction::Jiz("5".to_string()),
            Instruction::Ret,
        ];

        let bytecode = serialize_instructions(&original_instructions).unwrap();
        let decoded_instructions = deserialize_instructions(&bytecode).unwrap();

        assert_eq!(original_instructions, decoded_instructions);
    }
}
