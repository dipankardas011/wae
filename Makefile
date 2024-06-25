build_cli:
	@cd cli && cargo component build --release
	@wasm-tools compose cli/target/wasm32-wasi/release/cli.wasm -d crypto/crypto.wasm -o composed.wasm

build_crypto:
	@cd crypto && \
		componentize-py -d wit/world.wit -w crypto componentize --stub-wasi app_crypto -o crypto.wasm
		# python -m wasmtime.bindgen crypto.wasm --out-dir crypto_host

.PHONY: build
build: build_crypto build_cli
