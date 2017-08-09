Raspberry Pi LED blinker
========================

Compile
-------

You can either use Musle or standard toolchain. This have been tested only on ARMv7 architecture, however it should work on AArch64. You must first install the choosen toolchain via `rustup`, then compile with `cargo`. For instance:

```bash
cargo build --target=armv7-unknown-linux-gnueabihf --release
```
