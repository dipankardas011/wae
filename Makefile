.PHONY: build_cli
build_cli:
	@cd cli && cargo component build --release
	@wac plug cli/target/wasm32-wasi/release/cli.wasm \
		--plug crypto/crypto.wasm \
		--plug githubapi/githubapi.wasm \
		-o composed.wasm

.PHONY: build_crypto
build_crypto:
	@cd crypto && \
		componentize-py -d wit/world.wit -w crypto componentize app_crypto -o crypto.wasm

.PHONY: build_github_api
build_github_api:
	@cd githubapi && \
		componentize-py -d wit/world.wit -w project componentize app -o githubapi.wasm

.PHONY: build
build: build_crypto build_github_api build_cli

.PHONY: run_gen_pass
run_gen_pass:
	wasmtime run composed.wasm -n password-gen -o gen_rand_pass

.PHONY: run_demo
run_demo:
	wasmtime run --env OPENAI_API_KEY="ABCD1234" --dir=. composed.wasm -n dipankar -o demo

.PHONY: clean
clean:
	@rm -vrf cli/target crypto/crypto.wasm githubapi/githubapi.wasm
