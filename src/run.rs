use crate::instruction::Instruction;
use std::io::Write;

/// Executes a program of instructions and returns the final state of the stack and memory.
///
/// This is the main entry point for running Vortex VM programs. It processes each instruction
/// in sequence, maintaining a stack for data manipulation and a memory space for data storage.
///
/// # Examples
///
/// Basic stack operations:
///
/// ```
/// use vortex_vm::instruction::Instruction;
/// use vortex_vm::run::execute;
///
/// let program = vec![
///     Instruction::Push(5),
///     Instruction::Push(3),
///     Instruction::Add,
///     Instruction::Ret,
/// ];
///
/// let mut output = Vec::new();
/// let (stack, memory) = execute(&program, &mut output);
///
/// assert_eq!(stack, vec![8]);
/// assert_eq!(memory[0], 0); // Memory is initialized to zeros
/// ```
///
/// Memory operations:
///
/// ```
/// use vortex_vm::instruction::Instruction;
/// use vortex_vm::run::execute;
///
/// let program = vec![
///     Instruction::MemWrite(0, vec![72, 101, 108, 108, 111]), // "Hello"
///     Instruction::Print(0, 5),
///     Instruction::Ret,
/// ];
///
/// let mut output = Vec::new();
/// let (_stack, _memory) = execute(&program, &mut output);
///
/// let printed = String::from_utf8(output).unwrap();
/// assert_eq!(printed, "Hello");
/// ```
///
/// Jump instructions:
///
/// ```
/// use vortex_vm::instruction::Instruction;
/// use vortex_vm::run::execute;
///
/// let program = vec![
///     Instruction::Push(3),
///     Instruction::SubS(1),
///     Instruction::Jnz("1".to_string()), // Jump back to start if not zero
///     Instruction::Ret,
/// ];
///
/// let mut output = Vec::new();
/// let (stack, _memory) = execute(&program, &mut output);
///
/// assert_eq!(stack, vec![0]); // Should decrement from 3 to 0
/// ```
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
                i = execute_jiz(&stack, instructions, i, target);
            }
            Instruction::Jnz(target) => {
                i = execute_jnz(&stack, instructions, i, target);
            }
            Instruction::AddS(n) => {
                i = execute_adds(&mut stack, i, *n);
            }
            Instruction::Add => {
                i = execute_add(&mut stack, i);
            }
            Instruction::SubS(n) => {
                i = execute_subs(&mut stack, i, *n);
            }
            Instruction::Sub => {
                i = execute_sub(&mut stack, i);
            }
            Instruction::Dup => {
                i = execute_dup(&mut stack, i);
            }
            Instruction::Swap => {
                i = execute_swap(&mut stack, i);
            }
            Instruction::DivS(n) => {
                i = execute_divs(&mut stack, i, *n);
            }
            Instruction::Div => {
                i = execute_div(&mut stack, i);
            }
            Instruction::MultS(n) => {
                i = execute_mults(&mut stack, i, *n);
            }
            Instruction::Mult => {
                i = execute_mult(&mut stack, i);
            }
            Instruction::MemWrite(start_addr, values) => {
                i = execute_memwrite(&mut mem, i, *start_addr, values);
            }
            Instruction::Print(start_addr, length) => {
                i = execute_print(output_buffer, &mem, i, *start_addr, *length);
            }
            Instruction::MemRead(index) => {
                i = execute_memread(&mut stack, &mem, i, *index);
            }
            Instruction::MemWriteS(memory_index, write_len) => {
                i = execute_memwrites(&mut stack, &mut mem, i, *memory_index, *write_len);
            }
        }
    }

    (stack, mem)
}

// Jump instructions
fn execute_jiz(stack: &[i32], instructions: &[Instruction], current_i: usize, target: &str) -> usize {
    if let Some(&val) = stack.last()
        && val == 0
        && let Ok(addr) = target.parse::<usize>()
        && addr < instructions.len()
    {
        addr
    } else {
        current_i + 1
    }
}

fn execute_jnz(stack: &[i32], instructions: &[Instruction], current_i: usize, target: &str) -> usize {
    if let Some(&val) = stack.last()
        && val != 0
        && let Ok(addr) = target.parse::<usize>()
        && addr < instructions.len()
    {
        addr
    } else {
        current_i + 1
    }
}

// Arithmetic instructions
fn execute_adds(stack: &mut Vec<i32>, current_i: usize, n: i32) -> usize {
    if let Some(val) = stack.pop() {
        stack.push(val + n);
    }
    current_i + 1
}

fn execute_add(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b + a);
    }
    current_i + 1
}

fn execute_subs(stack: &mut Vec<i32>, current_i: usize, n: i32) -> usize {
    if let Some(val) = stack.pop() {
        stack.push(val - n);
    }
    current_i + 1
}

fn execute_sub(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b - a);
    }
    current_i + 1
}

fn execute_divs(stack: &mut [i32], current_i: usize, n: i32) -> usize {
    if let Some(val) = stack.last_mut() && n != 0 {
        *val /= n;
    }
    current_i + 1
}

fn execute_div(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        if a != 0 {
            stack.push(b / a);
        }
    }
    current_i + 1
}

fn execute_mults(stack: &mut [i32], current_i: usize, n: i32) -> usize {
    if let Some(val) = stack.last_mut() {
        *val *= n;
    }
    current_i + 1
}

fn execute_mult(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b * a);
    }
    current_i + 1
}

// Stack manipulation instructions
fn execute_dup(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if let Some(&val) = stack.last() {
        stack.push(val);
    }
    current_i + 1
}

fn execute_swap(stack: &mut Vec<i32>, current_i: usize) -> usize {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(a);
        stack.push(b);
    }
    current_i + 1
}

// Memory instructions
fn execute_memwrite(mem: &mut [i32], current_i: usize, start_addr: i32, values: &[i32]) -> usize {
    if start_addr < 2048 {
        for j in 0..values.len() {
            if (start_addr as usize + j) < mem.len() {
                mem[start_addr as usize + j] = values[j];
            }
        }
    }
    current_i + 1
}

fn execute_memwrites(stack: &mut Vec<i32>, mem: &mut [i32], current_i: usize, memory_index: i32, write_len: i32) -> usize {
    if memory_index as usize + write_len as usize <= mem.len() {
        let mut writes = Vec::with_capacity(write_len as usize);
        for _ in 0..write_len {
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
            mem[memory_index as usize + offset] = val;
        }
    } else {
        eprintln!("MemWriteS out of bounds at index {}", memory_index);
    }
    current_i + 1
}

fn execute_memread(stack: &mut Vec<i32>, mem: &[i32], current_i: usize, index: i32) -> usize {
    if index >= mem.len() as i32 {
        eprintln!("MemRead out of bounds: {}", index);
    } else {
        stack.push(mem[index as usize]);
    }
    current_i + 1
}

fn execute_print(output_buffer: &mut Vec<u8>, mem: &[i32], current_i: usize, start_addr: i32, length: i32) -> usize {
    let start = start_addr as usize;
    let end = start + length as usize;
    if end <= mem.len() {
        for &byte_val in mem.iter().take(end).skip(start) {
            write!(output_buffer, "{}", byte_val as u8 as char).unwrap();
        }
    } else {
        eprintln!("Print out of bounds: {}..{}", start, end);
    }
    current_i + 1
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
    fn test_null_instruction() {
        let program = vec![
            Instruction::Push(42),
            Instruction::Null, // Should do nothing
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![42]); // Stack should remain unchanged
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
    fn test_subtract() {
        let program = vec![
            Instruction::Push(10),
            Instruction::Push(3),
            Instruction::Sub, // 10 - 3 = 7
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![7]);
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
    fn test_jiz_jump() {
        let program = vec![
            Instruction::Push(0),
            Instruction::Jiz("3".to_string()), // Jump to RET if zero (which it is)
            Instruction::Push(99), // This should be skipped
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![0]); // Should not push 99
    }

    #[test]
    fn test_jiz_no_jump() {
        let program = vec![
            Instruction::Push(1),
            Instruction::Jiz("3".to_string()), // Don't jump if not zero
            Instruction::Push(99), // This should execute
            Instruction::Ret,
        ];
        let mut output = Vec::new();
        let (stack, _) = execute(&program, &mut output);
        assert_eq!(stack, vec![1, 99]); // Should push 99
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
