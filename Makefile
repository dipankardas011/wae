.PHONY: build_cli
build_cli:
	@cd cli && cargo component build --release
	@wasm-tools compose cli/target/wasm32-wasi/release/cli.wasm -d crypto/crypto.wasm -o composed.wasm

.PHONY: build_crypto
build_crypto:
	@cd crypto && \
		componentize-py -d wit/world.wit -w crypto componentize app_crypto -o crypto.wasm
		# python -m wasmtime.bindgen crypto.wasm --out-dir crypto_host

.PHONY: build
build: build_crypto build_cli

.PHONY: run_gen_pass
run_gen_pass:
	@wasmtime run composed.wasm -n password-gen -o gen_rand_pass

.PHONY: run_demo
run_demo:
	@wasmtime run --env OPENAI_API_KEY="ABCD1234" --dir=. composed.wasm -n dipankar -o demo
