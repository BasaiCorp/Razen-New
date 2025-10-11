# RAZE Implementation Master Plan

## Goal
Complete full IR instruction support in RAZE MIR and native code generation for x86_64 and ARM64.

## Current Status
✅ Basic arithmetic (Add, Subtract, Multiply)
✅ Stack operations (Push, Pop)
✅ Variable operations (Load, Store)
⚠️ Control flow (Jump, Label) - partial
❌ Comparisons - not implemented
❌ Logical operations - not implemented
❌ Bitwise operations - not implemented
❌ Arrays/Maps - not implemented
❌ I/O operations - not implemented

## Implementation Phases

### Phase 1: Core Arithmetic & Comparisons (2-3 instructions per commit)
**Priority: HIGH** - Needed for basic programs

#### Batch 1.1: Division & Modulo
- [ ] IR::Divide → MIR::DivInt/DivFloat
- [ ] IR::Modulo → MIR::ModInt
- [ ] x86_64: IDIV instruction
- [ ] ARM64: SDIV instruction

#### Batch 1.2: Power & FloorDiv
- [ ] IR::Power → MIR::Power (call to runtime helper)
- [ ] IR::FloorDiv → MIR::FloorDiv
- [ ] x86_64: Runtime call implementation
- [ ] ARM64: Runtime call implementation

#### Batch 1.3: Negate
- [ ] IR::Negate → MIR::NegInt/NegFloat
- [ ] x86_64: NEG instruction
- [ ] ARM64: NEG instruction

### Phase 2: Comparison Operations (2-3 instructions per commit)
**Priority: HIGH** - Needed for conditionals

#### Batch 2.1: Equal & NotEqual
- [ ] IR::Equal → MIR::CmpEqInt/CmpEqFloat
- [ ] IR::NotEqual → MIR::CmpNeInt/CmpNeFloat
- [ ] x86_64: CMP + SETE/SETNE
- [ ] ARM64: CMP + CSET

#### Batch 2.2: Greater comparisons
- [ ] IR::GreaterThan → MIR::CmpGtInt/CmpGtFloat
- [ ] IR::GreaterEqual → MIR::CmpGeInt/CmpGeFloat
- [ ] x86_64: CMP + SETG/SETGE
- [ ] ARM64: CMP + CSET

#### Batch 2.3: Less comparisons
- [ ] IR::LessThan → MIR::CmpLtInt/CmpLtFloat
- [ ] IR::LessEqual → MIR::CmpLeInt/CmpLeFloat
- [ ] x86_64: CMP + SETL/SETLE
- [ ] ARM64: CMP + CSET

### Phase 3: Control Flow (2-3 instructions per commit)
**Priority: CRITICAL** - Needed for any non-trivial program

#### Batch 3.1: Jumps & Labels
- [ ] IR::Jump → MIR::Jump with proper label resolution
- [ ] IR::Label → MIR::Label with address tracking
- [ ] x86_64: JMP with rel32 encoding
- [ ] ARM64: B instruction

#### Batch 3.2: Conditional Jumps
- [ ] IR::JumpIfFalse → MIR::JumpIfZero
- [ ] IR::JumpIfTrue → MIR::JumpIfNotZero
- [ ] x86_64: TEST + JZ/JNZ
- [ ] ARM64: CBZ/CBNZ

#### Batch 3.3: Function Calls & Return
- [ ] IR::Call → MIR::Call with argument passing
- [ ] IR::Return → MIR::Return with value
- [ ] x86_64: CALL/RET with stack frame
- [ ] ARM64: BL/RET with link register

### Phase 4: Logical Operations (2-3 instructions per commit)
**Priority: MEDIUM** - Common in conditionals

#### Batch 4.1: And & Or
- [ ] IR::And → MIR::And
- [ ] IR::Or → MIR::Or
- [ ] x86_64: AND/OR instructions
- [ ] ARM64: AND/ORR instructions

#### Batch 4.2: Not
- [ ] IR::Not → MIR::Not
- [ ] x86_64: XOR with 1 or NOT
- [ ] ARM64: MVN or EOR

### Phase 5: Bitwise Operations (2-3 instructions per commit)
**Priority: MEDIUM** - Useful for optimizations

#### Batch 5.1: BitwiseAnd & BitwiseOr
- [ ] IR::BitwiseAnd → MIR::BitAnd
- [ ] IR::BitwiseOr → MIR::BitOr
- [ ] x86_64: AND/OR instructions
- [ ] ARM64: AND/ORR instructions

#### Batch 5.2: BitwiseXor & BitwiseNot
- [ ] IR::BitwiseXor → MIR::BitXor
- [ ] IR::BitwiseNot → MIR::BitNot
- [ ] x86_64: XOR/NOT instructions
- [ ] ARM64: EOR/MVN instructions

#### Batch 5.3: Shifts
- [ ] IR::LeftShift → MIR::Shl
- [ ] IR::RightShift → MIR::Shr
- [ ] x86_64: SHL/SHR instructions
- [ ] ARM64: LSL/LSR instructions

### Phase 6: Stack Operations (2-3 instructions per commit)
**Priority: LOW** - Already partially working

#### Batch 6.1: Dup & Swap
- [ ] IR::Dup → MIR::Move (duplicate to new register)
- [ ] IR::Swap → MIR::Move sequence
- [ ] x86_64: MOV instructions
- [ ] ARM64: MOV instructions

### Phase 7: I/O Operations (2-3 instructions per commit)
**Priority: MEDIUM** - Needed for interactive programs

#### Batch 7.1: Print & Input
- [ ] IR::Print → MIR::Print (runtime call)
- [ ] IR::ReadInput → MIR::Input (runtime call)
- [ ] x86_64: CALL to runtime
- [ ] ARM64: BL to runtime

#### Batch 7.2: Exit
- [ ] IR::Exit → MIR::Exit (syscall or runtime)
- [ ] x86_64: Syscall or runtime
- [ ] ARM64: SVC or runtime

### Phase 8: Arrays & Maps (2-3 instructions per commit)
**Priority: LOW** - Complex data structures

#### Batch 8.1: Array Creation & Access
- [ ] IR::CreateArray → MIR::AllocArray
- [ ] IR::GetIndex → MIR::ArrayGet
- [ ] IR::SetIndex → MIR::ArraySet

#### Batch 8.2: Map Operations
- [ ] IR::CreateMap → MIR::AllocMap
- [ ] IR::GetKey → MIR::MapGet
- [ ] IR::SetKey → MIR::MapSet

### Phase 9: Advanced Features (2-3 instructions per commit)
**Priority: LOW** - Nice to have

#### Batch 9.1: Method Calls
- [ ] IR::MethodCall → MIR::Call with self parameter
- [ ] x86_64: CALL with adjusted arguments
- [ ] ARM64: BL with adjusted arguments

#### Batch 9.2: Exception Handling
- [ ] IR::SetupTryCatch → MIR exception setup
- [ ] IR::ClearTryCatch → MIR exception cleanup
- [ ] IR::ThrowException → MIR throw

#### Batch 9.3: Miscellaneous
- [ ] IR::Sleep → MIR::Sleep (runtime call)
- [ ] IR::LibraryCall → MIR::Call (dynamic dispatch)
- [ ] IR::DefineFunction → Function metadata

## Testing Strategy

After each batch:
1. Add test case in `test_raze.rzn`
2. Run `cargo run --release -- run --raze test_raze.rzn`
3. Verify output matches expected
4. Commit with descriptive message

## Commit Message Template
```
Implement RAZE [Phase X.Y]: [Feature Name]

Added MIR instructions:
- [Instruction 1]
- [Instruction 2]

Added x86_64 codegen:
- [Implementation details]

Added ARM64 codegen:
- [Implementation details]

Test: [What was tested]
Status: [Working/Partial/Needs work]
```

## Progress Tracking
- Total IR instructions: ~45
- Completed: ~8 (18%)
- Remaining: ~37 (82%)
- Estimated batches: 15-20
- Estimated time: 2-3 instructions per session

## Next Steps
1. Start with Phase 1, Batch 1.1 (Division & Modulo)
2. Implement in mir.rs translate_instruction
3. Implement in x86_64.rs generate_mir_instruction
4. Implement in aarch64.rs generate_mir_instruction
5. Test and commit
6. Move to next batch

---
Last Updated: 2025-10-11
Status: Planning Complete - Ready to implement
