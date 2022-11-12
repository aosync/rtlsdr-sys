# rtlsdr-sys

The `rtlsdr-sys` crate provides bindings over the C `librtlsdr` library.

The bindings were generated with [bindgen](https://github.com/rust-lang/rust-bindgen), and then were edited by hand.

## Dependencies

`librtlsdr` is included into the repository as a submodule and built by the `build.rs`
script, it is thus not necessary to have it installed on the system. However, `libusb`
must be installed on the system.