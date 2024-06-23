```bash
cargo component new cli --bin
```

How to get the environment variable getting passwd

```bash
wasmtime run --env OPENAI_API_KEY="${OPENAI_API_KEY}" --dir=. cli/target/wasm32-wasi/release/cli.wasm -n dipankar
```

```
Your Name: dipankar
OPENAI_API_KEY : XXYYZZ
In file README.md
With text:
# Automate Using WASM
...
```
