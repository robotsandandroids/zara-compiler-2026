# zara

A Rust project implementing a Zara language compiler.

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 2024 edition or newer)
- Git

### Building

```bash
cargo build
```

### Running the compiler

The compiler can process `.zara` files. Example files are available in the `examples/` directory:

```bash
cargo run -- examples/hello_world.zara
```

### Language grammar

Sample grammars for the syntax used by the examples are available in
[`examples/zara.bnf`](examples/zara.bnf) and [`examples/zara.ebnf`](examples/zara.ebnf).

### Running tests

```bash
cargo test
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

This project is licensed under the [Unlicense](LICENSE).