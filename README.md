# rust-wasm

> You can read Chinese document here [中文](./README_CN.md)

This is a simple example of how to use Rust and WebAssembly together to build a web application.

## Setup

You'll need to install Rust development tools. You can do this by running:

```sh
curl https://sh.rustup.rs -sSf | sh
```

## Running database

```sh
cd db
docker-compose up -d
```

Now you can access the database at `localhost:5432` with the username `postgres` and password `postgres`.

You can also access the database management tool at `localhost:8081` with the username `postgres` and
password `postgres` on the browser.

## Running webservice and webapp

Firstly, you should set the `DATABASE_URL` environment variable to include the database credentials like
so: `postgres://postgres:postgres@localhost:5432/postgres` on .env file of the project root.

Secondly, you should set the `HOST_PORT` environment variable to include the IP address and port of the host machine
like so: `http://localhost:8080` on .env file of the webapp root.

To run the webservice, you can run:

```sh
cd webservice
cargo run
```

Now you can open another terminal and run the webapp. To run the webapp, you can run:

```sh
cd webapp
cargo run
```

## Running WebAssembly

To run the WebAssembly, first you need to install the `wasm-pack` tool:

```sh
cargo install wasm-pack
```

Then, you can run the following commands:

```sh
cd wasm-client
wasm-pack build
```

install the npm dependencies:

```sh
cd wasm-client/www
npm install
```

run the WebAssembly app:

```sh
npm run start
```

## Build production

Build webservice and webapp:

```sh
cargo build --release --bin webservice
cargo build --release --bin webapp
```

Build WebAssembly app:

```sh
# generate WebAssembly package
cd wasm-client
wasm-pack build --release
# build webapp
cd www
npm run build
```
