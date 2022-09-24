# emul8: a simple chip-8 emulator
Powered by Rust and WGPU, emul8 is my first attempt at an emulator. It can compile for desktop platforms or the web!

![](https://i.imgur.com/wlBhBT6.png)
*emul8 running pong*

## Compiling for the web
To compile for the web, build for the target wasm32_unknown_unknown:
Set the RUSTFLAGS environment variable. In PowerShell, for example:
```ps
$env:RUSTFLAGS = "--cfg=web_sys_unstable_apis"
```
Then you can build:
```ps
cargo build --no-default-features --target wasm32-unknown-unknown
```
Finally, generate wasm bindings:
```
wasm-bindgen --out-dir web --web target/wasm32-unknown-unknown/debug/emul8.wasm
```
Now, serve the web folder, and it should run!