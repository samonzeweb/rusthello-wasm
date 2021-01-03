# rusthello-wasm

## Setup

It is possible to create WASM library with Rust without wasm-pack, lean more about Rust and WASM here : https://rustwasm.github.io/docs/book/ .

Install wasm-pack, see general instructions here : https://github.com/rustwasm/wasm-pack .

Due to [a bug in wasm-pack 0.9.1](https://github.com/rustwasm/wasm-pack/issues/837), install it from master until a new version is published :

```
cargo install --git https://github.com/rustwasm/wasm-pack.git
```

## Build

### Makefile

A makefile a available, simply run `make` and the project is built to be used with a bundler (like Webpack).

## Build for NodeJS and experiment

```
rm pkg/*
wasm-pack build --target=nodejs
```

Start NodeJS with `node` and try things like this :

```
const rusthello = require('./pkg/rusthello_wasm')
let game = rusthello.WGame.new(rusthello.WPlayer.Black, 6)
game.game_over()
game.winner()
game.opponent_is_blocked()
game.count_black()
game.count_white()
game.player_play(4,5)
let move = game.computer_play()
move.x
move.y
game.log()
game.player_play(0,0) // bad move
.exit
```

## Manual build for the web

This option allow the use of the build directly in web content, without bundler like webpack.

```
rm pkg/*
wasm-pack build --target web
```

## Manual build for a bundler

This option allow the use of Webpack. WASM is supported since Webpack 4, but getting all running together could be chalanging due to some requirment about WASM loading. I struggled to tame the beast ([like other peaople](https://github.com/rustwasm/wasm-bindgen/issues/700)), until the light came from this comment : https://github.com/webpack/webpack/issues/6615#issuecomment-668177931 .

```
rm pkg/*
wasm-pack build --target bundler
wasm-pack pack
```

