# Native
```
cargo run
```

# Build wasm pkg
```
cd pathfinder/web_canvas/
wasm-pack build
```
if success, the `web_canvas` will generate `pkg` dir. if hang on `installing wasm-bindgen ...`.
Please manually install `wasm-bindgen`
```
cargo install wasm-bindgen-cli --version 0.2.64
```
then rebuild `wasm`

# JavaScript use
```
cd js
npm install
npm run start #start serve
```
