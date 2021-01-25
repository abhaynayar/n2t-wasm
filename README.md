## n2t-wasm

Emulator for the [nand2tetris](https://www.nand2tetris.org/) computer (built
using Rust and WebAssembly).

### Screenshot

![](screenshots/wasm-demo2.png)

### Demo

https://abhaynayar.com/n2t-wasm/

### Backlog

- [ ] Create separate targets for wasm and pixels. Shouldn't have to comment code.
- [ ] _Feature:_ Add keyboard support.
- [ ] _Bug:_ Fix the render loop in wasm.
- [ ] _Bug:_ Sanitize "Insert RAM" inputs.
- [ ] _Bug:_ Index of instruction should be within _ROM_ size.
- [ ] _Bug:_ Crash when `Rect.hack` is running with `RAM[0] > 513`.
- [ ] _Feature:_ Transfer words instead of pixels from wasm to js.
- [ ] _Feature:_ Show sample _ROMs_ and a file picker on the web page.
