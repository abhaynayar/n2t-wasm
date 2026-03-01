## n2t-wasm

Emulator for the Hack CPU. (from [nand2tetris](https://www.nand2tetris.org/))

### Demo

https://abhaynayar.github.io/n2t-wasm/

You can try the demo with this "Pong" Hack file that I found online: [Pong.hack](https://github.com/jopdorp/nand2tetris-verilog/blob/39cbde1f61c10fb97a0da329831e74c50d129519/Pong.hack). Unfortunately, the emulator lacks keyboard support, preventing gameplay. Additionally, the unoptimized emulator results in a slow performance. This project only served as an introductory learning exercise for me in WebAssembly.

If you'd like to try other Hack programs, you can search for them online, or learn Jack to make your own. You don’t need to build the entire toolchain; my [Genesis](https://github.com/abhaynayar/genesis) repository already has that. (compiler, VM, assembler). Write a Jack program, compile it with Genesis, and run the resulting Hack machine code here.


### Build Instructions

Tested on Ubuntu 24.04 LTS.

Tools setup:
- Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install C build toolchain: `sudo apt install -y build-essential pkg-config libssl-dev`
- Install wasm target: `rustup target add wasm32-unknown-unknown`
- Install wasm-bindgen CLI: `cargo install wasm-bindgen-cli`

Clone this repository:

```
$ git clone https://github.com/abhaynayar/n2t-wasm
```

Build and run the emulator:

- Run `./run.sh` at the repository root to build Rust (`wasm32-unknown-unknown`), generate JS bindings in `www/pkg/`, and start a local web server.
- Open `http://localhost:8080`.

### Backlog

- [ ] Feature: Add keyboard support.
- [x] Bug: Fix the render loop in wasm.
- [ ] Bug: Sanitize RAM and ROM inputs in wasm.
- [x] Feature: Transfer words instead of pixels from wasm to js.
- [x] Feature: Show sample _ROMs_ and a file picker on the web page.
