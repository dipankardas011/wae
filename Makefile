# Set the color variable
green=\033[0;32m
# Clear the color after that
clear=\033[0m

.PHONY: gen-componentize-py-crypto
gen-componentize-py-crypto:
	cd crypto && rm -rf crypto && \
		componentize-py --wit-path wit/world.wit --world crypto bindings .

.PHONY: gen-componentize-py-githubapi
gen-componentize-py-githubapi:
	cd githubapi && rm -rf githubapi && \
		componentize-py --wit-path wit --world project bindings .

.PHONY: build_cli
build_cli:
	cd cli && \
		cargo component build --release
	@echo -e "${green}PASS${clear} Build for cli/"
	wac plug cli/target/wasm32-wasi/release/cli.wasm \
		--plug crypto/crypto.wasm \
		--plug githubapi-composed.wasm \
		-o composed.wasm
	@echo -e "${green}PASS${clear} wac plug for cli/"

.PHONY: build_crypto
build_crypto:
	cd crypto && \
		componentize-py \
			-d wit/world.wit \
			-w crypto \
			componentize app_crypto \
			-o crypto.wasm
	@echo -e "${green}PASS${clear} Build for crypto/"

.PHONY: build_github_api
build_github_api:
	cd githubapi && \
		componentize-py \
			-d wit \
			-w project \
			componentize app \
			-o githubapi.wasm
	@echo -e "${green}PASS${clear} Build for githubapi/"
	wac plug githubapi/githubapi.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o githubapi-composed.wasm
	@echo -e "${green}PASS${clear} wac plug for githubapi/"

.PHONY: build_httpclient
build_httpclient:
	cd httpclient && \
		cargo component build --release
	@echo -e "${green}PASS${clear} Build for httpclient/"

.PHONY: build
build: build_httpclient build_crypto build_github_api build_cli
	@echo -e "${green}DONE${clear} Build all the components"
	@echo -e "next run the following commands make run_* to run the components"

.PHONY: run_gen_pass
run_gen_pass:
	wasmtime run -S cli -S http composed.wasm -n password-gen -o gen_rand_pass

.PHONY: run_demo
run_demo:
	wasmtime run -S cli -S http --env OPENAI_API_KEY="ABCD1234" --dir=. composed.wasm -n dipankar -o demo

.PHONY: run_get_latest_release
run_get_latest_release:
	wasmtime run -S http composed.wasm -n dipankar -o proj_latest_release

.PHONY: clean
clean:
	rm -vrf \
		cli/target \
		crypto/crypto.wasm \
		githubapi/githubapi.wasm \
		httpclient/target \
		wasihttpclient/target \
		composed.wasm \
		githubapi-composed.wasm
	@echo -e "${green}DONE${clear} removed all the compiled files"
