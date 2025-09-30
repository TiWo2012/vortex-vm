# Roadmap for Stack-Based VM in Rust

## 🎯 Goal (v1)
Build a stack-based VM in Rust that can:
- Perform arithmetic (`Add`, `Sub`, `Mult`, `Div`)
- Manipulate the stack (`Push`, `Pop`, `Dup`, `Swap`)
- Control execution (`Jiz`, `Jnz`, `Ret`)
- Support both “immediate” and “stack” versions of math ops (`Add(i32)` vs. `Add` using stack).

---

## 🛠️ Phase 1 — Core VM Engine
- [x] Define the `VM` struct with:
  - `stack: Vec<i32>`
  - `ip: usize` (instruction pointer)
  - `program: Vec<Instruction>`
- [x] Implement `run()` loop until `Ret` or program end.

---

## 📦 Phase 2 — Stack Operations
- [x] Implement `Push(i32)` (push immediate values).
- [x] Implement `Pop` (pop top value).
- [x] Implement `Dup` (duplicate top).
- [x] Implement `Swap` (swap top 2).
- [x] Unit tests for all of the above.

---

## ➕ Phase 3 — Arithmetic
- [x] Implement **immediate forms**: `Add(i32)`, `Sub(i32)`, `MultS(i32)`, `DivS(i32)` → pop top, apply with given operand, push result.
- [x] Implement **stack forms**: `Add`, `Sub`, `Mult`, `Div` → pop top 2 values, apply, push result.
- [x] Handle division by zero safely.
- [x] Unit tests: simple arithmetic programs.

---

## 🔁 Phase 4 — Control Flow
- [x] Implement `Jiz(i32)` (jump if zero) → pop top, if 0 then jump.
- [x] Implement `Jnz(i32)` (jump if not zero).
- [x] Implement `Ret` (terminate program).
- [x] Test small loop programs (`while x != 0 { x -= 1 }`).

---

## 🖨️ Phase 5 — Output / Debugging
- [ ] Temporary `Print` function (not an instruction, just for debugging).
- [ ] Add a “trace mode”: before each `step()`, print `ip`, current instr, stack.
- [ ] Write a few end-to-end examples (factorial, sum of numbers).

---

## 🚀 Phase 6 — Extras (Optional)
- [ ] Labels in source code (`:loop` → resolved to instruction index).
- [ ] Simple assembler: take a text file (`PUSH 5; PUSH 3; ADD`) → `Vec<Instruction>`.
- [ ] Functions (`Call`, `Ret` with call stack).
- [ ] Extended types (bools, strings).
