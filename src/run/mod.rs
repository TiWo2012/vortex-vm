use crate::instruction::Instruction;

fn execute(instructions: &[Instruction]) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
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
                        if (*target as usize) < instructions.len() {
                            i = *target as usize;
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                i += 1;
            }
            Instruction::Jnz(target) => {
                if let Some(&val) = stack.last() {
                    if val != 0 {
                        if (*target as usize) < instructions.len() {
                            i = *target as usize;
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                i += 1;
            }
            Instruction::Add(n) => {
                if let Some(&val) = stack.last() {
                    stack.push(val + n);
                }
                i += 1;
            }
            Instruction::Sub(n) => {
                if let Some(&val) = stack.last() {
                    stack.push(val - n);
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
        }
    }

    stack
}

pub fn run(instructions: &[Instruction]) {
    let stack = execute(instructions);
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_push_and_add() {
        let program = vec![Instruction::Push(5), Instruction::Add(3), Instruction::Ret];
        let stack = execute(&program);
        assert_eq!(stack, vec![5, 8]);
    }

    #[test]
    fn test_push_pop() {
        let program = vec![Instruction::Push(10), Instruction::Pop, Instruction::Ret];
        let stack = execute(&program);
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
        let stack = execute(&program);
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
        let stack = execute(&program);
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
        let stack = execute(&program);
        assert_eq!(stack, vec![4, 2]);
    }
}
