# rlox: Lox Interpreter / REPL in Rust

[![Rust Build & Test](https://github.com/WenzeJin/rlox/actions/workflows/rust.yml/badge.svg)](https://github.com/WenzeJin/rlox/actions/workflows/rust.yml)  

`rlox` is a Rust implementation of the Lox programming language, as described in the book [Crafting Interpreters](http://craftinginterpreters.com/). It includes a REPL (Read-Eval-Print Loop) and the ability to execute Lox scripts.

## Features

- A fully functional Lox interpreter written in Rust.
- Support for both interactive REPL and script execution.
- Implements the Lox language as described in *Crafting Interpreters*.
- Easy to build, run, and test using `cargo`.
- Well tested with unit tests and integration tests.
- Will support List and Map

## Usage

### Running the REPL

To start the interactive REPL:

```bash
./rlox
```

### Running a Lox Script

To execute a Lox script:

```bash
./rlox <script>
```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/WenzeJin/rlox.git
   cd rlox
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the REPL or a script as described in the [Usage](#usage) section.

## How to Build & Run

To build the project in release mode:

```bash
cargo build --release
```

To build and run the project:

```bash
cargo run
```

## How to Test

Run the test suite using `cargo`:

```bash
cargo test
```

## Contributing

Contributions are welcome! If you'd like to contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

> [!NOTE] 
> Copyright (c) 2025 [Wenze Jin](https://wenzejin.github.io)