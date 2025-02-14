🏅 ritmo

<img alt="A rounded black square with a white plus-symbol on it." src="public/icon/ritmo.png" width="180px"/>

A habit tracker web app written in [Rust](https://www.rust-lang.org/) with [Leptos](https://leptos.dev/).

## Features

- ✅ Track habits on a daily basis
- 👤 No Login
- 🔌 Works offline
- 📁 Data ex- and import

## Development

Install [trunk](https://github.com/trunk-rs/trunk) for building the web app and [leptosfmt](https://github.com/bram209/leptosfmt) for code-formatting.  
Then add the wasm target and start the development server.

```sh
cargo install trunk leptosfmt
rustup target add wasm32-unknown-unknown
trunk serve --port 3000
```

## Build

Building the app will ouput all necessary files to the `dist` directory. Choose whatever static file server you like to serve them.

```sh
trunk build --release
```
