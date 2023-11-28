# Knowledge and Reasoning: Reversi

The game Reversi (aka Othello) (see [the rules](https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english)) made in Rust on the Bevy game engine to demonstrate the `MinMax` algorithm.

## Play in browser

You can run the game on the [github pages deployment](https://laytongb.github.io/kar-reversi-minmax/).

## Compile locally (Windows and WASM only)

To compile locally, make sure Rust is installed and up to date, then on windows run:

```rs
cargo install just
just build-game
```

When building has finished the executable will be at `target\release\kar_reversi_minmax.exe`.

## Play in terminal

The back-end API is fully operational from the terminal and performant enough to run without optimizations. 

You can try that out by running:

```rs
cargo run
```

## Notes

The `Async` algorithm was an attempt to speed up the computation time but actually slows the system down! This is because in making the processes asynchronous, all of the comparisons are made before any alpha-beta-pruning is performed, leading to much greater work even though the game is distributing the work better.
