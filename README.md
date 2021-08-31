# Rusty Bot

Rusty bot is a bot to play connect4. It is written in Rust, but compiles to wasm with a thin javascript interface, to be consumed by a web application.

The core of the bot logic is minimax tree search with alpha-beta pruning.

## To Build

```shell
./build.sh
```

## To Test

```shell
cargo test
```
