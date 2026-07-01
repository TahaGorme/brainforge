# brainforge

A Brainfuck compiler written in Rust.

The goal of this project is to build a compiler from the ground up without relying on parser generators or compiler frameworks. Every stage of the compilation pipeline is implemented manually, starting with lexical analysis and eventually progressing through parsing, validation, optimization, and code generation.

The project is still a work in progress and is under active development.

## Features

Currently implemented:

- Manual lexer over `std::io::Read`
- Ignores non-Brainfuck characters
- Source location tracking (line and column)
- Single-token lookahead (`peek()`)
- Error handling using `anyhow`
- File-based source input

## Architecture

The compiler currently consists of a single stage.

```
Source File
    |
    v
BufReader
    |
    v
Lexer
    |
    v
Tokens
```

The lexer reads the source one byte at a time, filtering everything except the eight valid Brainfuck instructions:

```text
<>+-.,[]
```

Each instruction is emitted as a `Token` containing both the instruction itself and its location in the source file. Source locations are preserved to make future parser and compiler diagnostics more useful.

## Planned

The remaining compiler pipeline will include:

- Parser
- Bracket matching and jump resolution
- Semantic validation
- Intermediate representation (if needed)
- Code generation
- Executable output
- Compiler diagnostics with source spans
- Tests
- Optimization passes

## Running

```sh
cargo run -- examples/hello_world.b
```

## Current Status

- [x] Project setup
- [x] File loading
- [x] Lexer
- [x] Token stream
- [x] Source location tracking
- [ ] Parser
- [ ] Bracket matching
- [ ] Semantic analysis
- [ ] Intermediate representation
- [ ] Optimization passes
- [ ] Code generation
- [ ] Executable output
- [ ] Tests
- [ ] Documentation

The lexer is largely complete and serves as the foundation for the rest of the compiler. The parser and code generation stages are the next major milestones.

---

This project is primarily a compiler construction exercise and a way to explore Rust, parsing techniques, and low-level code generation by building everything from scratch.
