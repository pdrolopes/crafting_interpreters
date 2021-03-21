# Crafting Interpreters - Rust Implementation

A personal Rust implementation of the interpreter presented by Robert Nystrom in his book [Crafting Interpreters](https://craftinginterpreters.com/).

## Usage

### Install Rust

`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`  
`$ rustup install nightly`

[Detailed instructions](https://doc.rust-lang.org/book/ch01-01-installation.html)

### Download the project

`$ git clone https://github.com/pdrolopes/crafting_interpreters.git`

### Build the project

`$ cd crafting_interpreters`  
`$ cargo +nightly build`

### Running

`$ ./target/debug/crafting_interpreters`

### Writing your program

`> print "Hello World";`  
`Hello World`

## The Lox Language

The interpreter reads the **Lox language**, created by Nystrom.  
You can find more details about it on [this link](https://craftinginterpreters.com/the-lox-language.html).
