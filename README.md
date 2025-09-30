# vortex-vm  

A lightweight **32-bit stack-based virtual machine** written in Rust.  
It executes a small instruction set designed for experimentation, learning, and simple program execution.  

---

## Features  
- **Stack-based execution** model  
- **32-bit values**  
- **Case-insensitive instructions**  
- **Basic arithmetic & control flow**  
- Simple, extensible instruction set  

---

## Instruction Set  

All instructions are **case-insensitive**.  

| Instruction | Description |
|-------------|-------------|
| `NULL`      | No-op, just increments the instruction pointer. |
| `PUSH <n>`  | Pushes an integer value onto the stack. |
| `POP`       | Pops (removes) the top value from the stack. |
| `RET`       | Returns, halting execution. Leaves the topmost value on the stack as the result. |
| `JNZ <n>`   | Jumps to instruction `<n>` if the top value is **not zero**. |
| `JIZ <n>`   | Jumps to instruction `<n>` if the top value is **zero**. |
| `ADD <n>`   | Adds `<n>` to the topmost value on the stack. |
| `SUB <n>`   | Subtracts `<n>` from the topmost value on the stack. |
| `MULT`      | Multiplies the top two values on the stack. |
| `MULTS <n>` | Multiplies the topmost value with `<n>`. |
| `DIV`       | Divides the second-top value by the top value. |
| `DIVS <n>`  | Divides the topmost value by `<n>`. |
| `DUP`       | Duplicates the top value on the stack. |
| `SWAP`      | Swaps the top two values on the stack. |

---

## Example Program  

This simple program calculates `5 * 4` using a loop (repeated addition). 
**Note:** the instructions are case-insensitive

```text
PUSH 0      ; result = 0
PUSH 5      ; counter = 5

; loop start (index 2)
JIZ 7       ; if counter == 0 -> jump to end
PUSH 4
ADD 0       ; result = result + 4
SUB 1       ; counter = counter - 1
JNZ 2       ; repeat until counter == 0

; end
RET
```

**Execution result:**  
```
[20]
```

---

## Running  

Clone the repo and run:  

```bash
cargo run -- examples/loop_mult.vvm
```

---

## Roadmap  

[Roadmap](./vm_roadmap.md)
---

## License  

MIT – feel free to use, modify, and distribute.  
