build_cli:
	@cd cli && cargo component build --release

run_cli:
	@wasmtime cli/target/wasm32-wasi/release/cli.wasm
