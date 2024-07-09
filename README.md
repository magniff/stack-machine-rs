# Type-Level Virtual Machine in Rust
## YouTube clip
Watch me writing that code [here](https://youtu.be/gQMVG_5zG1M?si=YzPXHiigErUYHheF)

## Overview

This project implements a type-level virtual machine (VM) in Rust, showcasing the power and expressiveness of Rust's type system. By leveraging advanced type system features, we've created a stack-based VM that performs computations at compile-time.

## Features

- Peano number representation at the type level
- Type-level arithmetic operations (addition, multiplication)
- Stack-based VM with various instructions
- Compile-time computation
- Demonstration of advanced Rust type system features

## How It Works

The VM is implemented entirely using Rust's type system. It includes:

1. **Peano Numbers**: Natural numbers represented as nested types (`Zero`, `Suc<Zero>`, `Suc<Suc<Zero>>`, etc.)
2. **VM Instructions**: Including `NOOP`, `ADD`, `DUP`, `ROT3`, `JUMP`, and `PUSH`
3. **VM State**: Represented by a stack, instruction set, and instruction pointer
4. **Execution**: Modeled through trait implementations

## Key Components

- `Number` trait: Marks types that represent numbers
- `NormalForm` trait: Defines the normal form of a number
- `Add` and `Mul` structs: Represent addition and multiplication operations
- `VMState` struct: Represents the state of the virtual machine
- `Fetch` trait: Retrieves the current instruction
- `Exec` trait: Defines how each instruction modifies the VM state

## Main Example: Fibonacci Sequence

Here's how you can use this type-level VM to compute the Fibonacci sequence, watch how the type of the `vm` thingy changes with each new `step()` added to the chain: 

```rust
#[test]
fn test_fib() {
    // Fibonacci sequence: 1, 1, 2, 3, 5, 8, 13, 21, ...
    let vm: VMState
        Stack<Empty, Empty, Empty, Empty>,
        Instructions<PUSH<One>, PUSH<Two>, DUP, ROT3, ADD, JUMP<Two>, NOOP, NOOP>,
        Zero,
    > = VMState {
        _phantom: std::marker::PhantomData,
    };
    
    let vm = vm
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step();
}
```
