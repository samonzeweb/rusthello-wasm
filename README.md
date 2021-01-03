# Rusthello-WASM

## Introduction

This projet was my learning path to begin with [Rust](https://www.rust-lang.org) and [WebAssembly (WASM)](https://webassembly.org/). Nothing impressive, just some toy code to discover and experiment.

As a Rust beginner I started from an simple (naive) implementation of Othello game in the projet [rusthello](rusthello/README.md). Then I was tempted to experiment with WASM and made the [rusthello-wasm](rusthello-wasm/README.md). And since I had what it took to get the game to work in a browser I used Vue to build a web frontend in [rusthello-web](rusthello-web/README.md). This is called going down the rabbit hole, and it was wonderful 🙂

The result is visible here : https://rusthello-wasm.netlify.app . I'm not a frontend guy, and the purpose was not to build a shining-production-ready product, it's just code for fun.

## Going down the rabbit hole

### What you need

* the Rust toolchain : https://www.rust-lang.org/tools/install .
* wask-pack : see [rusthello-wasm setup](rusthello-wasm/README.md#Setup) .
* [NodeJS](https://nodejs.org) with [npm](https://www.npmjs.com) .

### Build

Simply run `make`, and get the result in `rusthello-web/dist`.

### Explore

Each project has its own README with some more informations.

## Licence

Released under the MIT License, see [LICENSE](LICENSE) file for more informations.