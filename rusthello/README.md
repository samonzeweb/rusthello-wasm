# rusthello

## Introduction

Rusthello is an implementation of the Othello (or Reversi) game,
its purpose is to practice a little [Rust](https://www.rust-lang.org/).

It's a learning project, some parts are quick & dirty, tests are sometimes limited,
and some features are used for the sole purpose of... using them (`Cell`, feature flag, ...).

The build produce an executable, and and independant library.

## Setup

Install Rust toolchain if needed : https://www.rust-lang.org/tools/install

## Build

A Makefile is available, simply run `make` and the projet is tested and built. After the build, the executable is here : `target/release/rusthello`.

### Manual build and run.

Run :

```
cargo build --release
```

To execute the game you have to give your color, and the depth of exploration for
the virtual player (deeper = slower), ie :

```
target/release/rusthello black 6
```

Usage :

```
Usage : ./rusthello color depth
  color : 'black' or 'white'
  depth : 4 .. 10 (more than 8 could be slow)
```

### Run in debug mode

Exemple :

```
cargo run -- black 6
```

### Test

Fast tests : `cargo test`.

All tests (slower) : `cargo test --features alphabetavsminimax`.

All tests, showing stats for minimax vs alphabeta : `cargo test --features alphabetavsminimax -- --nocapture`.
