build_cli:
	@cd cli && cargo component build --release

build-crypto:
	@cd crypto && \
		componentize-py -d wit/world.wit -w password componentize --stub-wasi app_password -o password.wasm && \
		python -m wasmtime.bindgen password.wasm --out-dir password_host

