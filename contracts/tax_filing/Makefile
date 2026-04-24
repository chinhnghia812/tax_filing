rust:
	cargo build --target wasm32-unknown-unknown --release
	wasm32-unknown-unknown-objcopy --strip-all target/wasm32-unknown-unknown/release/tax_filing.wasm target/wasm32-unknown-unknown/release/tax_filing.optimized.wasm

test:
	cargo test

clean:
	cargo clean
	rm -f target/wasm32-unknown-unknown/release/tax_filing.optimized.wasm
