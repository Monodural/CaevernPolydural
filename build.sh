cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/caevern_polydural.wasm ../CaevernClient/assets/modules/
cd ../CaevernClient
cargo run
