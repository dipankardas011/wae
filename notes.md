## Some notes on wasm compilation

Wasmtime needs to AOT compile the Wasm to native code before running it, and when there's a lot of code that takes a long time.  It caches the result on disk, which is why subsequent runs are faster.
For serverless systems like Fermyon Cloud, "cold" means "starting from a .cwasm file", i.e. the already-compiled native code.  We only have to do that step once at deployment time, and we keep it around until the app is deleted.
If "cold" means starting from a .wasm file, then yes, it's slow
