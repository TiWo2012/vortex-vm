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
                stack.pop(); // ignore underflow
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
                            break; // invalid jump target
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
            Instruction::Add(target) => {
                if let Some(&val) = stack.last() {
                    stack.push(val + target);
                }
                i += 1;
            }
            Instruction::Sub(target) => {
                if let Some(&val) = stack.last() {
                    stack.push(val - target);
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
            Instruction::DivS(target) => {
                if let Some(&val) = stack.last() {
                    if *target != 0 {
                        stack.push(val / target);
                    }
                }
                i += 1;
            }
            Instruction::Div => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if a != 0 {
                        let res = b / a;
                        stack.push(b);
                        stack.push(a);
                        stack.push(res);
                    }
                }
                i += 1;
            }
            Instruction::MultS(target) => {
                if let Some(&val) = stack.last() {
                    stack.push(val * target);
                }
                i += 1;
            }
            Instruction::Mult => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let res = b * a;
                    stack.push(b);
                    stack.push(a);
                    stack.push(res);
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
            Instruction::Swap,
            Instruction::Dup,
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
            Instruction::Mult,
            Instruction::Dup,
            Instruction::Div,
            Instruction::Ret,
        ];

        let stack = execute(&program);
        assert_eq!(stack, vec![1, 25, 25, 25, 1]);
    }

    #[test]
    fn test_mults_and_divs() {
        let program = vec![
            Instruction::Push(2),
            Instruction::MultS(2),
            Instruction::Dup,
            Instruction::DivS(2),
        ];

        let stack = execute(&program);
        assert_eq!(stack, vec![2, 4, 4, 2]);
    }
}
