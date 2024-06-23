build_cli:
	@cd cli && cargo component build --release

build-crypto:
	@cd crypto && componentize-py -d wit/world.wit -w crypto componentize --stub-wasi password -o crypto.wasm
