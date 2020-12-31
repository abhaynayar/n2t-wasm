TODO:

- [ ] Add system.init() support? Is it needed or will it be compiled?
- [ ] Test operations on 16 bit limits & fix all the signedness issues.
- [ ] Should we make js and wasm to operate on their own? (eg. updating pc)
- [ ] Transfer words to js instead of pixels. (for debug support)

- [ ] Add keyboard support.
- [ ] Add breakpoint support.
- [ ] Upload files throught a file picker on the web page.
- [ ] Create wasm interfaces for debugger tools on the web page.

BUGS:

- [ ] If Rect.hack is running with RAM[0] > 513, things crash.
