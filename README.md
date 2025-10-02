# vortex-vm  

A lightweight **32-bit stack-based virtual machine** written in Rust.  
It executes a small instruction set designed for experimentation, learning, and simple program execution.  

---

## Features
- **Stack-based execution** model
- **32-bit signed integer values**
- **Case-insensitive instructions**
- **Label-based jumps** for readable control flow
- **Inline comments** with `;` character
- **Basic arithmetic & control flow**
- **Memory operations** with 2048 addressable locations
- **I/O operations** for character output
- Simple, extensible instruction set

---

## Instruction Set

All instructions are **case-insensitive** and support **inline comments** using `;`.

### Stack Operations
| Instruction | Description |
|-------------|-------------|
| `NULL`      | No-op, just increments the instruction pointer. |
| `PUSH <n>`  | Pushes a 32-bit signed integer value onto the stack. |
| `POP`       | Pops (removes) the top value from the stack. |
| `DUP`       | Duplicates the top value on the stack. |
| `SWAP`      | Swaps the top two values on the stack. |

### Control Flow
| Instruction | Description |
|-------------|-------------|
| `RET`       | Returns, halting execution. Leaves the stack unchanged. |
| `JNZ <addr>`| Jumps to instruction at `<addr>` (numeric) or label if the top value is **not zero**. Pops the value. |
| `JIZ <addr>`| Jumps to instruction at `<addr>` (numeric) or label if the top value is **zero**. Pops the value. |

### Arithmetic Operations
| Instruction | Description |
|-------------|-------------|
| `ADD`       | Pops two values, adds them (second + first), pushes result. |
| `ADDS <n>`  | Adds `<n>` to the topmost value on the stack (in-place). |
| `SUB`       | Pops two values, subtracts them (second - first), pushes result. |
| `SUBS <n>`  | Subtracts `<n>` from the topmost value on the stack (in-place). |
| `MULT`      | Pops two values, multiplies them (second * first), pushes result. |
| `MULTS <n>` | Multiplies the topmost value by `<n>` (in-place). |
| `DIV`       | Pops two values, divides them (second / first), pushes result. Division by zero is ignored. |
| `DIVS <n>`  | Divides the topmost value by `<n>` (in-place). Division by zero is ignored. |

### Memory Operations
| Instruction | Description |
|-------------|-------------|
| `MEMWRITE <addr> <val1> <val2> ...` | Writes multiple values to memory starting at `<addr>`. |
| `MEMWRITES <addr> <len>` | Pops `<len>` values from stack and writes them to memory starting at `<addr>`. |
| `MEMREAD <addr>` | Reads a value from memory at `<addr>` and pushes it onto the stack. |
| `PRINT <addr> <len>` | Prints `<len>` characters from memory starting at `<addr>` to stdout. |

---

## Label Support

Labels provide readable jump targets and are resolved during parsing:

```assembly
main:
    PUSH 5
    JIZ end_loop    ; Jump if zero to 'end_loop'
    SUBS 1
    JNZ main       ; Jump back to 'main' label
end_loop:
    RET
```

---

## Comment Support

Both traditional and inline comments are supported:

```assembly
; Traditional comment style
PUSH 42
; Another comment

; Inline comment style
PUSH 42 ; This is an inline comment
ADD 8   ; Another inline comment
RET
```

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
