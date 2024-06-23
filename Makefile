build_cli:
	@cd cli && cargo component build --release

build-crypto:
	@cd crypto && componentize-py --wit-path wit/world.wit --world crypto componentize password -o crypto.wasm
