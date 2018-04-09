# Color Me Rusty

A demonstration in Rust + WebAssembly to set the background color of a webpage

## Files

```
/color-me-rusty
|-- /src
|   |-- main.rs
|   |-- colors.rs
|
|-- /target
|   |-- /wasm32-unknown-unknown
|   |-- |-- /release
|   |-- |-- |-- color-me-rusty.wasm
|
|-- Cargo.toml
|-- Cargo.lock
|-- color-me-rusty.gc.wasm
|-- index.html
|-- index.js
|-- README.md
```

## Setup

### Installing the Rustup toolchain

```
$ curl https://sh.rustup.rs -sSf | sh
```

*See also: [The Rust Programming Language Book](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)*

### Installing Rust Nightly

```
$ rustup toolchain install nightly
```

### Updating Rust Nightly

```
$ rustup update
```

### Installing the WebAssembly target

```
$ rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Installing wasm-gc

```
$ cargo install --git https://github.com/alexcrichton/wasm-gc
```

## Compilation

### Compiling Rust to WebAssembly

```
$ cargo +nightly build --target wasm32-unknown-unknown --release
```

### Creating a smaller binary with wasm-gc

```
$ wasm-gc target/wasm32-unknown-unknown/release/color-me-rusty.wasm -o color-me-rusty.gc.wasm
```

## Resources

* [Hello Rust](https://hellorust.com)
* [Rust + WASM Book](https://rust-lang-nursery.github.io/rust-wasm/)

## Acknowledgements

Thank you to the following people for contributing ideas and enthusiasm and/or pairing with me!

* [alexcoco](https://github.com/alexcoco)
* [cfinucane](https://github.com/cfinucane)
* [connorwalsh](https://github.com/connorwalsh)
* [mkcor](https://github.com/mkcor)
* [sarahwelzgeselowitz](https://github.com/sarahwelzgeselowitz)
