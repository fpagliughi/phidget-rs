# Change Log for phidget-rs library crate


## [v0.1.2]  - unreleased

- Seperated voltage-io to voltage-input and ouput. (Allows better implementation of available functions from binding)
- Seperated digital-io to digital-input and ouput. (Allows better implementation of available functions from binding)
- Completion of available functions from in digital-input and ouput binding.
- Moved devices to specific module to keep the root directory clean.
- Renamed `build.rs` env. variable `TARGET` to `PHIDGET_ROOT` to point to the directory of phidgets22, as well as adding a search to allow rust to find anything else it would need.

## [v0.1.1](https://github.com/fpagliughi/phidget-rs/compare/v0.1.0..v0.1.1)  - 2023-04-20

- Added attach/detach setup functions for the devices
- Removed generic callback setters from `Phidget` trait to allow it to be used as an object, like `Vec<Box<Phidget>>`
- Phidgets implement `Send` trait
- Added a `GenericPhidget` type
- Downgraded clap to v3.2 (from 4.x) to allow for lower/older MRSV (now Rust v0.59.0)

## [v0.1.0](https://github.com/fpagliughi/phidget-rs/tree/v0.1.0) - 2023-04-19

Initial release with basic support for the following sensors:

- Temperature
- Humidity
- Voltage in/out
- Digital in/out
- Hubs