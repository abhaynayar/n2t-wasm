## n2t-wasm

Emulator for the [nand2tetris](https://www.nand2tetris.org/) computer (built
using Rust and WebAssembly).

### Demo

https://abhaynayar.com/n2t-wasm/

- The `Rect.hack` ROM is preloaded into the webpage.
- First click on `Load` to load the **ROM** into the emulator.
- Then **Store** the height of the rectangle at address 0 in the **RAM**.
- Finally hit the `>>` button.
- You'll see a rectangle get drawn on the screen!



### Build

- Clone the repository `git clone https://github.com/abhaynayar/n2t-wasm`
- Set up the required tools using [this](https://rustwasm.github.io/docs/book/game-of-life/setup.html) tutorial.
- Run `wasm-pack build` in the root directory to build the wasm modules.
- Run `npm run start` in the `www` directory to start a local testing instance.
- Run `npm run build` in the `www` directory to publish the results to `dist`.

### Todo

- [ ] Right now, the emulator runs very slow as it bottlenecks on
    `requestAnimationFrame`. However, when the loop is implemented in wasm, it
    runs fast, but crashes the browser. So we need to figure out better way to
    implement the render loop.
- [ ] Fix: Index of instruction to be executed should be within ROM size.
- [ ] Fix: Crash when `Rect.hack` is running with RAM[0] > 513.
- [ ] Show sample ROMs and a file picker on the web page.
- [ ] Add keyboard support.
