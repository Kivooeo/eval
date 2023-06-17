# eval-rs

eval-rs is a simple command-line tool written in Rust that evaluates mathematical expressions. It allows you to substitute variables, provides built-in constants such as pi and e, and supports various mathematical functions like cosine, sine, etc. This project serves as an introductory project for learning the Rust programming language and using Git for version control.

## Features

- Evaluate basic mathematical expressions
- Substitute variables in expression
- Support for built-in constants (pi, e, etc.)
- Support for mathematical functions (cos, sin, etc.)
- Can round your final answer

## Installation

To use test or using this program you need to have Rust and Cargo installed on your system. If you don`t have them, you can install them on [https://rustup.rs/](https://rustup.rs/)

Once you have Rust and Cargo installed, you can clone this repository:

```rust
git clone https://github.com/Kivooeo/eval-rs/
```

then go inside cloned directory 

```rust
cd eval-rs
```

and build my project using Cargo

```rust
cargo build --release
```

## Usage

You can use eval-rs by running compiled binary and providing mathematical expression as a command-line argument. For example

```rust
./target/release/eval
// Basic mathematical expressions
>>> 2 + 2
answer is 4
// Using constant and operating them
>>> pi
answer is 3.1415926536
>>> pi + 2
answer is 5.1415926536
>>> pi ^ e - 1
answer is 21.459157717041357
// Filling expression with variables
>>> x + y | 2 4
answer is 6
// Round answer
>>> pi | | 1
answer is 3.1
// Something more complex 
// (btw this cant help you to solution complex expressions like this one sqrt(-1) ) 
>>> cos(pi)
answer is -1
>>> sqrt(cos(pi))
answer is NaN
>>>
```

## What's new at version 1.2

- Simple support for range based expressions
- Fixed pow sign priority
it worked like this `2 ^ 3 ^ 4 == (2 ^ 3) ^ 4`
now it fixed and work like this `2 ^ 3 ^ 4 == 2 ^ (3 ^ 4)`

```rust
//little about range based
>>> (1..5).add()
answer is 10.00
>>> (1..5).sub()
answer is -9.00
>>> (1..=5).mul()
answer is 120.000
>>> (2 * 2..4).add() // cuz sum of range 4..4 == 0
answer is 0.0
>>> (cos(pi)..5).add() // also can use constants and trig. functions inside if they return integer like `cos(pi) == -1`
answer is 9.0
>>> (x..y).add() | 1 5
answer is 10.00
>>> (x..y).add() * 2 + 2 | 1 5
answer is 22.00
>>>
```
## Contributing

If you`re interested to contributing to eval-rs, feel free to fork the repository, make your changes and submit a pull request. We welcome any contributions, whether they are bug fixes, new features, or improvements.

Before submitting a pull request, please ensure that your code passes the existing tests and add any tests as necessary.

---

## License

This project is under MIT License. See the LICENSE file for more details.

## Acknowledgments

- The Rust programming language: **[https://www.rust-lang.org/](https://www.rust-lang.org/)**
- Git documentation: **[https://git-scm.com/doc](https://git-scm.com/doc)**
