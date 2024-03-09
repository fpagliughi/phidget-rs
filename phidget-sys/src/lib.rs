// phidget-sys/src/lib.rs
//
//!
//!

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Temporary
#![allow(dead_code)]

// Bindgen uses u128 on some rare parameters
#![allow(improper_ctypes)]

// Bring in the bindings for phidget22
#[cfg(all(target_pointer_width = "64"))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/phidget22-64.rs"));

#[cfg(all(target_pointer_width = "32"))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/phidget22-32.rs"));