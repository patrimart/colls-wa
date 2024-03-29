# colls-wa

A library with high performance collections written in Rust for wasm.

Available Collections:

- Ordered Set

## Problems

1. Webpack converts the dynamic `import` to `document.createElement('script')`.
   This prevents the library from being used in a web worker. Need to preserve `import`.
   May just need to write vanilla JS and avoid Webpack.

## How to install

```sh
yarn
```

## Build wasm JS interface

```sh
# Builds the wasm file and JS bindings in /pkg (imported from /js).
# (Recommend) Global install wasm-pack
wasm-pack build
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
yarn build
```

## How to run unit tests

```sh
# Runs tests in Firefox
yarn test -- --firefox

# Runs tests in Chrome
yarn test -- --chrome

# Runs tests in Safari
yarn test -- --safari
```

## What does each file do?

- `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

- `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

- `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

- The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

- The `src` folder contains your Rust code.

- The `tests` folder contains your Rust unit tests.
