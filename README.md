# Hand Compiler

A compiler for the .hand programming language that generates C code.

## Installation

Build the compiler in release mode:

```bash
cargo build --release
```

## Usage

Compile a .hand source file to C:

```bash
./target/release/compiler source.hand
```

This generates a `source.c` file that can be compiled with any C compiler:

```bash
gcc -o program source.c
./program
```

## Example Program

```hand
let i = 0;
let sum = 0;

loop 10 {
    i = i + 1;
    sum = sum + i;
    print sum;
};
```

This will print the cumulative sum: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55.
