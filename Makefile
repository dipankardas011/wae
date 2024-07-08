# Set the color variable
green=\033[0;32m
# Clear the color after that
clear=\033[0m

.PHONY: gen-componentize-py-crypto
gen-componentize-py-crypto:
	cd crypto && rm -rf crypto && \
		componentize-py --wit-path wit/world.wit --world crypto bindings .


.PHONY: gen-componentize-py-watttime
gen-componentize-py-watttime:
	cd watttime && \
		wit-deps && \
		rm -rf green && \
		componentize-py --wit-path wit --world green bindings .

.PHONY: gen-componentize-py-githubapi
gen-componentize-py-githubapi:
	cd githubapi && \
		wit-deps && \
		rm -rf project && \
		componentize-py --wit-path wit --world project bindings .

.PHONY: gen-componentize-py-openai
gen-componentize-py-openai:
	cd openai && \
		wit-deps && \
		rm -rf genai && \
		componentize-py --wit-path wit --world genai bindings .

.PHONY: build_cli
build_cli:
	cd cli && \
		cargo component build --release
	@echo -e "${green}PASS${clear} Build for cli/"
	wac plug cli/target/wasm32-wasi/release/cli.wasm \
		--plug crypto/crypto.wasm \
		--plug githubapi-composed.wasm \
		--plug watttime-composed.wasm \
		--plug openai-composed.wasm \
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

.PHONY: build_watttime
build_watttime:
	cd watttime && \
		componentize-py \
			-d wit \
			-w green \
			componentize app \
			-o watttime.wasm
	@echo -e "${green}PASS${clear} Build for watttime/"
	wac plug watttime/watttime.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o watttime-composed.wasm
	@echo -e "${green}PASS${clear} wac plug for watttime/"

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

.PHONY: build_openai
build_openai:
	cd openai && \
		componentize-py \
			-d wit \
			-w genai \
			componentize app \
			-o openai.wasm
	@echo -e "${green}PASS${clear} Build for openai/"
	wac plug openai/openai.wasm \
		--plug httpclient/target/wasm32-wasi/release/httpclient.wasm \
		-o openai-composed.wasm
	@echo -e "${green}PASS${clear} wac plug for openai/"

.PHONY: build_httpclient
build_httpclient:
	cd httpclient && \
		cargo component build --release
	@echo -e "${green}PASS${clear} Build for httpclient/"

.PHONY: build
build: build_httpclient build_crypto build_openai build_watttime build_github_api build_cli
	@echo -e "${green}DONE${clear} Converting the wasm -> cwasm for native bindings"
	wasmtime compile composed.wasm
	@echo -e "${green}DONE${clear} Build all the components"
	@echo -e "next run the following commands make run_* to run the components"

.PHONY: run_gen_pass
run_gen_pass:
	wasmtime run -S cli -S http --allow-precompiled composed.cwasm -n password-gen -o crypto

.PHONY: run_get_latest_release
run_get_latest_release:
	wasmtime run -S http --allow-precompiled composed.cwasm -n dipankar --op githubapi

.PHONY: run_openai
run_openai:
	wasmtime run -S http --allow-precompiled --dir=. composed.cwasm -n dipankar --op openai

.PHONY: run_green
run_green:
	wasmtime run -S http --allow-precompiled --dir=. composed.cwasm -n dipankar --op green

.PHONY: run_server
run_server:
	wasmtime serve -O pooling-allocator=n -S cli -S http --allow-precompiled --dir=. composed.cwasm

.PHONY: clean
clean:
	rm -vrf \
		cli/target \
		crypto/crypto.wasm \
		githubapi/githubapi.wasm \
		watttime/watttime.wasm \
		openai/openai.wasm \
		httpclient/target \
		wasihttpclient/target \
		composed.wasm \
		composed.cwasm \
		githubapi-composed.wasm \
		watttime-composed.wasm \
		openai-composed.wasm
	@echo -e "${green}DONE${clear} removed all the compiled files"
