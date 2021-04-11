## n2t-wasm

Emulator for the [nand2tetris](https://www.nand2tetris.org/) computer.

### Screenshot

![](screenshots/wasm-demo2.png)

### Demo

https://abhaynayar.com/n2t-wasm/

### Build Instructions

Tested on Ubuntu 20.04 LTS.

Tools setup:
- Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- Install cargo-generate: `cargo install cargo-generate`
- Install node through nvm:

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
nvm install node
```

Clone this repository:

```
$ git clone https://github.com/abhaynayar/n2t-wasm
```

Building the emulator:
- Run `wasm-pack build` in the root directory. This will create a new pkg/ directory and build the wasm modules in it.
- Run `npm init wasm-app www` in the root directory. This will generate a web page in a new www/ directory. This is already done so you don't need to do it.
- Run `npm install` in www directory to install the node modules.
- Run `npm run start` in the `www` directory to start a local testing instance.
- Run `npm run build` in the `www` directory to publish the results to `dist`.

### Backlog

- [ ] Bug: Sanitize "Insert RAM" inputs.
- [ ] Bug: Better keyboard responsiveness.
- [ ] Bug: Index of instruction should be within _ROM_ size.
- [ ] Bug: Crash when `Rect.hack` is running with `RAM[0] > 513`.
- [ ] Feature: Show sample _ROMs_ and a file picker on the web page.
- [ ] Feature: Separate targets for wasm and pixels. Shouldn't have to comment code.

- [x] Feature: Add keyboard support.
- [x] Bug: Fix the render loop in wasm.
- [x] Feature: Transfer words instead of pixels from wasm to js.
- [x] Bug: Only first key press is being registered. (=> keymap had to be zeroed)
