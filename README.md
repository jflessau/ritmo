# 🏅 ritmo

<img alt="A rounded black square with a white plus-symbol on it." src="public/icon/ritmo.png" width="180px"/>

Habit tracker web app written in [Rust](https://www.rust-lang.org/) with [Leptos](https://leptos.dev/).

## Features

- ✅ Track habits on a daily basis
- 👤 No Login
- 🔌 Works offline
- 📁 Data ex- and import
- 🌐 PWA ready

👉 [Demo](https://ritmo.jflessau.com)

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

Or use the provided Dockerfile to build a docker image:

```sh
docker build -t ritmo .
docker run -p 8080:80 ritmo
```
