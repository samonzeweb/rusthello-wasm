# rusthello-web

## Introduction

The project is based Vue 3 and Typescript and was created with [Vue CLI](https://cli.vuejs.org/).

As it's a learning project, there is something to ... learn. The project was developed with Vite due to problems between webpack and the wasm-pack build, then migrated back to Vue CLI (using webpack) when I solved it. The problem was this error :

```
WebAssembly module is included in initial chunk.
This is not allowed, because WebAssembly download and compilation must happen asynchronous.
Add an async splitpoint (i. e. import()) somewhere between your entrypoint and the WebAssembly module:
```

To reproduce the problem, simply add `import "rusthello-wasm";` into the entrypoint
 (here it is `src/index.js`). But who would want to do that ?

 The key point is to have an entrypoint wich load all asynchronously, see [this comment](https://github.com/webpack/webpack/issues/6615#issuecomment-668177931).

For this project I simply ...

* Added `vue.config.js` to change de entrypoint.
* Added `src/index.js` as the new entrypoint.
* et voilà !

## Setup

This project require :
* [NodeJS](https://nodejs.org) with [npm](https://www.npmjs.com).
* [rusthello-wasm](../rusthello-wasm/README.md) build to be used with a bundler.

## Build

### Makefile

Simply run `make`. If you rebuild the project an error could appear, but will not block the build process. See [troubleshooting](#Troubleshooting) below.

The result is in the `dist` folder and can be deployed as a static web site.

## Run manually in development

```
npm i
npm run serve
```

## Build manually (production)

```
npm i
npm run build
```

## Troubleshooting

As `rusthello-wasm` is referenced in the local filesystem instead of a public repository, if you build `rusthello-wasm` multiple times, `npm` become really unpleasant and the projet will not be built. The brutal way to solve the problem is :

```bash
# inside the current project, not rusthello-wasm.
rm -Rf node_modules && npm i || npm i
```
