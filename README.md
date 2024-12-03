## n2t-wasm

Emulator for the Hack CPU. (from [nand2tetris](https://www.nand2tetris.org/))

### Demo

https://abhaynayar.github.io/n2t-wasm/

You can try the demo with this "Pong" Hack file that I found online: [Pong.hack](https://github.com/jopdorp/nand2tetris-verilog/blob/39cbde1f61c10fb97a0da329831e74c50d129519/Pong.hack). Unfortunately, the emulator lacks keyboard support, preventing gameplay. Additionally, the unoptimized emulator results in a slow performance. This project only served as an introductory learning exercise for me in WebAssembly.

If you'd like to try other Hack programs, you can search for them online, or learn Jack to make your own. You don’t need to build the entire toolchain; my [Genesis](https://github.com/abhaynayar/genesis) repository already has that. (compiler, VM, assembler). Write a Jack program, compile it with Genesis, and run the resulting Hack machine code here.


### Build Instructions

Tested on Ubuntu 20.04 LTS.

Tools setup:
- Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- Install node through nvm:

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
nvm install node
```

Clone this repository:

```
$ git clone https://github.com/abhaynayar/n2t-wasm
```

Build the emulator:

- Run `wasm-pack build` in the root directory. This will create a new pkg/ directory for the wasm modules.
- Run `npm init wasm-app www` in the root directory. This will generate a web page in a new www/ directory.
- Run `npm install` in www directory to install the node modules.
- Run `npm run start` in the `www` directory to start a local testing instance.
- Run `npm run build` in the `www` directory to publish the results to `dist`.

### Backlog

- [ ] Feature: Add keyboard support.
- [x] Bug: Fix the render loop in wasm.
- [ ] Bug: Sanitize RAM and ROM inputs in wasm.
- [x] Feature: Transfer words instead of pixels from wasm to js.
- [x] Feature: Show sample _ROMs_ and a file picker on the web page.
