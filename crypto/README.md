
it creates the initial boilderplate code using the wit
```bash
componentize-py --wit-path wit/world.wit --world password bindings password_guest
```


once done we will make a compoent out
```bash
componentize-py -d wit/world.wit -w password componentize --stub-wasi app_password -o password.wasm
```

how to use host thing


```bash
pip install wasmtime
python -m wasmtime.bindgen password.wasm --out-dir password_host

cat <<EOF > host.py
from password_host import Root
from wasmtime import Config, Engine, Store

config = Config()
config.cache = True
engine = Engine(config)
store = Store(engine)
hello = Root(store)
print(f"component says: {hello.generate_random(store, 5)}")
EOF
```

