home-easy-rs
============

![Travis CI](https://travis-ci.org/NSenaud/home-easy-rs.svg?branch=master)

Description
-----------

`home-easy-rs` aims to implement the home-easy protocol, used by some home automation devices. Currently, it have been tested with Chacon [DIO remote plugs](https://www.leroymerlin.fr/v3/p/produits/lot-de-3-prises-telecommandables-interieure-dio-e183624) (Ref. 54760), a Seeds Studio 433MHz emetter and a Raspberry Pi 3 on ArchLinux. The current code is mostly a translation of the C++ code of [Vincent Demay](http://www.homautomation.org/2013/10/09/how-to-control-di-o-devices-with-a-raspberry/) and [Idleman](http://blog.idleman.fr/raspberry-pi-10-commander-le-raspberry-pi-par-radio/).

This code produce both a libray to use with your own projects, and a CLI utility called `dios` to test it easily.

Cross-Compile
-------------

You must first choose a toolchain to cross-compile. Only ARMv7 architecture on Raspberry Pi have been tested yet, however it should work on AArch64. You must first install the choosen toolchain via `rustup`, then compile with `cargo`. For instance:

```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf --release
```

CLI Utility Usage
-----------------

You must first authentify the emetter with the receipter (the way to do it depends on your device, please refer to its manual).

```bash
sudo dios 0 12321234 3 on -v
```
