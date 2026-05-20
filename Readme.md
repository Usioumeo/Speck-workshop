# Installation:
- if not present install a c compiler (build-essential or similar)
- Install rustup following the instruction on this page: https://rustup.rs/
- install cargo-binstall (in order to install the other tools faster, without recompiling): https://github.com/cargo-bins/cargo-binstall
- install espup, espflash (helper to install esp toolchain, and flasher): cargo binstall espup espflash
- install esp toolchain: espup install

### How to compile
load the env for the toolchain: . $HOME/export-esp.sh 
inside the repo, you can execute: cargo build

### how to flash
inside the repo, you can execute: cargo run



# Repository structure
normal cargo project, 

# useful links:
- rust book: https://doc.rust-lang.org/book/
- rust on esp book: https://docs.espressif.com/projects/rust/book/
- esp-generate: https://github.com/esp-rs/esp-generate
- esp-hal: https://github.com/esp-rs/esp-hal


### disclaimer
Obviusly rust-analyzer broke with the specialized toolchain esp. This means that we need to pin an old version of rust-analyzer to work (until the next version of esp toolchain)
https://github.com/rust-lang/rust-analyzer/issues/22371