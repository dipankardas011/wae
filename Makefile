.PHONY: build_cliv2
build_cliv2:
	cd cli && \
		cargo component build --release
	@echo "PASS: [+] Build for cli/"
	wac plug cli/target/wasm32-wasi/release/cli.wasm \
		--plug crypto/crypto.wasm \
		--plug githubapiv2-composed.wasm \
		-o composed.wasm
	@echo "PASS: [+] wac plug for cli/"

.PHONY: build_cli
build_cli:
	cd cli && \
		cargo component build --release
	@echo "PASS: [+] Build for cli/"
	wac plug cli/target/wasm32-wasi/release/cli.wasm \
		--plug crypto/crypto.wasm \
		--plug githubapi-composed.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o composed.wasm
	@echo "PASS: [+] wac plug for cli/"

.PHONY: build_crypto
build_crypto:
	cd crypto && \
		componentize-py \
			-d wit/world.wit \
			-w crypto \
			componentize app_crypto \
			-o crypto.wasm
	@echo "PASS: [+] Build for crypto/"

.PHONY: build_github_apiv2
build_github_apiv2:
	cd githubapiv2 && \
		componentize-py \
			-d wit \
			-w project \
			componentize app \
			-o githubapiv2.wasm
	@echo "PASS: [+] Build for githubapiv2/"
	wac plug githubapiv2/githubapiv2.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o githubapiv2-composed.wasm
	@echo "PASS: [+] wac plug for githubapiv2/"

.PHONY: build_github_api
build_github_api:
	cd githubapi && \
		cargo component build --release
	@echo "PASS: [+] Build for githubapi/"
	wac plug githubapi/target/wasm32-wasi/release/githubapi.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o githubapi-composed.wasm
	@echo "PASS: [+] wac plug for githubapi/"

.PHONY: build_httpclient
build_httpclient:
	cd httpclient && \
		cargo component build --release
	@echo "PASS: [+] Build for httpclient/"

.PHONY: build
build: build_httpclient build_crypto build_github_api build_cli

.PHONY: buildv2
buildv2: build_httpclient build_crypto build_github_apiv2 build_cliv2

.PHONY: run_gen_pass
run_gen_pass:
	wasmtime run composed.wasm -n password-gen -o gen_rand_pass

.PHONY: run_demo
run_demo:
	wasmtime run --env OPENAI_API_KEY="ABCD1234" --dir=. composed.wasm -n dipankar -o demo

.PHONY: run_get_latest_release
run_get_latest_release:
	wasmtime run -S http composed.wasm -n dipankar -o proj_latest_release

.PHONY: clean
clean:
	rm -vrf \
		cli/target \
		crypto/crypto.wasm \
		githubapi/target \
		githubapiv2/githubapiv2.wasm \
		httpclient/target \
		wasihttpclient/target \
		composed.wasm \
		githubapi-composed.wasm \
		githubapiv2-composed.wasm
