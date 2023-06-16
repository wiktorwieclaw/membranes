# membranes
NES emulator written in Rust.

## Compile membranes to wasm
```
wasm-pack build --target=web membranes -d membranes-www/pkg
```

## Run membranes-www
```
http-server membranes-www
```
