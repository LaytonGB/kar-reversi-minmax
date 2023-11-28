# use powershell not sh
set shell := ["powershell.exe", "-c"] 

# run standalone game in dev mode
run-game:
    cargo run --no-default-features -F game,bevy/dynamic_linking

# package wasm into docs folder
package-wasm:
    wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/kar_reversi_minmax.wasm

# build all targets
build-all:
    cargo build --release
    cargo build --release --no-default-features -F game
    cargo build --release --no-default-features -F game --target wasm32-unknown-unknown

# build all targets and package wasm to docs folder
build-and-package:
    just build-all
    just package-wasm
