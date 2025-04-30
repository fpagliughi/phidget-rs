# phidget-sys

Low-level unsafe wrpper around the `phidget22` library for interacting with [Phidget](https://www.phidgets.com/) devices.

It was originally created for use with the [phidget](https://crates.io/crates/phidget) crate.

This is primarily a set of [bindgen](https://crates.io/crates/bindgen)-generated bindings of the `phidget22.h` header file and linkage to the library.


To regenerate bindings, use a recent version of _bindgen_ like this:

```
$ bindgen --rust-target <MSRV> --no-doc-comments phidget22.h > bindings/phidget22-XX.rs
```

where:
    - **MSRV** is the Minimum Supported Rust version (currently v1.73.0), and
    - **XX** is the word size on the platform (**64** or **32**)

So, to generate 64-bit bindings for the current MSRV:

```
$ bindgen --rust-target 1.73.0 --no-doc-comments phidget22.h > bindings/phidget22-64.rs
```
