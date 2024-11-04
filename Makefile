.PHONY: build
build:
	@cargo build --release --target wasm32-unknown-unknown
	@wasm-bindgen target/wasm32-unknown-unknown/release/rust_wasm_bindgen_example.wasm --out-dir ./out
