use crate::instruction::Instruction;
use std::io::Write;

pub fn execute(instructions: &[Instruction], output_buffer: &mut Vec<u8>) -> (Vec<i32>, Vec<i32>) {
    let mut stack: Vec<i32> = Vec::new();
    let mut mem: Vec<i32> = vec![0; 2048];
    let mut i: usize = 0;

    while i < instructions.len() {
        match &instructions[i] {
            Instruction::Null => {
                i += 1;
            }
            Instruction::Push(value) => {
                stack.push(*value);
                i += 1;
            }
            Instruction::Pop => {
                stack.pop();
                i += 1;
            }
            Instruction::Ret => {
                break;
            }
            Instruction::Jiz(target) => {
                if let Some(&val) = stack.last() {
                    if val == 0 {
                        if let Ok(addr) = target.parse::<usize>() {
                            if addr < instructions.len() {
                                i = addr;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
                i += 1;
            }
            Instruction::Jnz(target) => {
                if let Some(&val) = stack.last() {
                    if val != 0 {
                        if let Ok(addr) = target.parse::<usize>() {
                            if addr < instructions.len() {
                                i = addr;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
                i += 1;
            }
            Instruction::AddS(n) => {
                if let Some(val) = stack.pop() {
                    stack.push(val + n);
                }
                i += 1;
            }
            Instruction::Add => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b + a);
                }
                i += 1;
            }
            Instruction::SubS(n) => {
                if let Some(val) = stack.pop() {
                    stack.push(val - n);
                }
                i += 1;
            }
            Instruction::Sub => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                }
                i += 1;
            }
            Instruction::Dup => {
                if let Some(&val) = stack.last() {
                    stack.push(val);
                }
                i += 1;
            }
            Instruction::Swap => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a);
                    stack.push(b);
                }
                i += 1;
            }
            Instruction::DivS(n) => {
                if let Some(val) = stack.last_mut() {
                    if *n != 0 {
                        *val /= n;
                    }
                }
                i += 1;
            }
            Instruction::Div => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if a != 0 {
                        stack.push(b / a);
                    }
                }
                i += 1;
            }
            Instruction::MultS(n) => {
                if let Some(val) = stack.last_mut() {
                    *val *= n;
                }
                i += 1;
            }
            Instruction::Mult => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);
                }
                i += 1;
            }
            Instruction::MemWrite(start_addr, values) => {
                if *start_addr < 2048 {
                    for j in 0..values.len() {
                        if (*start_addr as usize + j) < mem.len() {
                            mem[*start_addr as usize + j] = values[j];
                        }
                    }
                }
                i += 1;
            }
            Instruction::Print(start_addr, length) => {
                let start = *start_addr as usize;
                let end = start + *length as usize;
                if end <= mem.len() {
                    for idx in start..end {
                        write!(output_buffer, "{}", mem[idx] as u8 as char).unwrap();
                    }
                } else {
                    eprintln!("Print out of bounds: {}..{}", start, end);
                }
                i += 1;
            }
            Instruction::MemRead(index) => {
                if *index >= mem.len() as i32 {
                    eprintln!("Print out of bounds: {}", index);
                }

                stack.push(mem[*index as usize]);

                i += 1;
            }
            Instruction::MemWriteS(memory_index, write_len) => {
                if *memory_index as usize + *write_len as usize <= mem.len() {
                    let mut writes = Vec::with_capacity(*write_len as usize);
                    for _ in 0..*write_len {
                        if let Some(val) = stack.pop() {
                            writes.push(val);
                        } else {
                            eprintln!("Stack underflow on MemWriteS");
                            break;
                        }
                    }
                    // Reverse because stack pop order is backwards
                    writes.reverse();

                    for (offset, val) in writes.into_iter().enumerate() {
                        mem[*memory_index as usize + offset] = val;
                    }
                } else {
                    eprintln!("MemWriteS out of bounds at index {}", memory_index);
                }
                i += 1; // 🔥 advance instruction pointer
            }
        }
    }

    (stack, mem)
}

pub fn run(instructions: &[Instruction]) {
    let mut output = Vec::new();
    let stack = execute(instructions, &mut output);
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_memwrites() {
        let program = vec![
            Instruction::Push(5),
            Instruction::Dup,
            Instruction::Dup,
            Instruction::Dup,
            Instruction::MemWriteS(0, 4),
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, memory) = execute(&program, &mut output);
        let mut expected_memory = vec![0; 2048];
        expected_memory[0] = 5;
        expected_memory[1] = 5;
        expected_memory[2] = 5;
        expected_memory[3] = 5;
        assert_eq!(stack, vec![]);
        assert_eq!(memory, expected_memory)
    }

    #[test]
    fn test_push_and_add() {
        let program = vec![Instruction::Push(5), Instruction::AddS(3), Instruction::Ret];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![8]);
    }

    #[test]
    fn test_push_pop() {
        let program = vec![Instruction::Push(10), Instruction::Pop, Instruction::Ret];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_dup_and_swap() {
        let program = vec![
            Instruction::Push(1),
            Instruction::Push(2),
            Instruction::Swap, // stack: [2,1]
            Instruction::Dup,  // stack: [2,1,1]
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![2, 1, 1]);
    }

    #[test]
    fn test_mult_and_div() {
        let program = vec![
            Instruction::Push(1),
            Instruction::Push(25),
            Instruction::Mult, // [25]
            Instruction::Dup,  // [25,25]
            Instruction::Div,  // [1]
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![1]);
    }

    #[test]
    fn test_mults_and_divs() {
        let program = vec![
            Instruction::Push(2),
            Instruction::MultS(2), // [4]
            Instruction::Dup,      // [4,4]
            Instruction::DivS(2),  // [4,2]
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![4, 2]);
    }

    #[test]
    fn test_loop_program() {
        let program = vec![
            Instruction::Push(5),
            Instruction::SubS(1),
            Instruction::Jnz("1".to_string()),
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![0]);
    }

    #[test]
    fn test_mem_write() {
        let program = vec![
            Instruction::Push(0),
            Instruction::MemWrite(0, vec![1, 1, 1, 1]),
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, mem) = execute(&program, &mut output);
        let predicted_stack = vec![0];
        let mut predicted_mem = vec![0; 2048];
        predicted_mem[0] = 1;
        predicted_mem[1] = 1;
        predicted_mem[2] = 1;
        predicted_mem[3] = 1;

        assert_eq!(stack, predicted_stack);
        assert_eq!(mem, predicted_mem);
    }

    #[test]
    fn test_mem_read() {
        let program = vec![
            Instruction::MemWrite(0, vec![1, 2, 3, 4]),
            Instruction::MemRead(0),
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, mem) = execute(&program, &mut output);
        let predicted_stack = vec![1];
        let mut predicted_mem = vec![0; 2048];
        predicted_mem[0] = 1;
        predicted_mem[1] = 2;
        predicted_mem[2] = 3;
        predicted_mem[3] = 4;

        assert_eq!(stack, predicted_stack);
        assert_eq!(mem, predicted_mem);
    }

    #[test]
    fn test_print() {
        let program = vec![
            Instruction::MemWrite(0, vec![72, 101, 108, 108, 111, 33]), // "Hello!"
            Instruction::Print(0, 6),
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (_stack, _mem) = execute(&program, &mut output);
        let printed = String::from_utf8(output).unwrap();
        assert_eq!(printed, "Hello!");
    }
}
