# Automate Using WASM

Why create this?
Becuase this is the main selling point of the wasi to be able to create logic indenendent of the language and highly decoplied ways and with low latency and low footprint (Env Sustabanle ;>)

## Skills to be achieved
- use of wasm in real world uscase of FAAS
- python openai,
- rust cli
- watttime api
- python

## How to use it 
```shell
make build
make run_**
```

## Dependency graph

```mermaid
graph TD;
    cli(["`CLI Written **Rust**`"])-->crypto(["`Crypto Written **Python**`"]);
    cli-->githubapi(["`Githubapi Written **Python**`"]);
    cli-->openai(["`OpenAI LLM Written **Python**`"]);
    cli-->watt(["`Watt_time client Written **Python**`"]);
    githubapi-->httpclient(["`HTTPClient Written **Rust**`"]);
    watt-->httpclient;
    openai-->httpclient;
    httpclient-->wasi:http;
```

> `httpclient` uses `waki` crate for the http client

`wasihttpclient` uses the `wasi:http` crate for the http client which is actually used by `waki`
