https://github.com/bytecodealliance/wasmtime-py?tab=readme-ov-file#components

initlaize the deps in wit file manually via the `wit-deps`
wrote to wit/deps.toml

$ wit-deps update

$ componentize-py -d wit -w project bindings githubapiv2_guest
$ componentize-py -d wit -w project componentize app -o githubapiv2.wasm
