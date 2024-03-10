# Change Log for phidget-rs library crate


## [v0.1.2](https://github.com/fpagliughi/phidget-rs/compare/v0.1.1..v0.1.2)  - 2024-03-10

- Bumped MSRV to 1.73
- Seperated voltage-io to voltage-input and ouput modules.
- Seperated digital-io to digital-input and ouput modules.
- Completion of available functions from in digital-input and ouput binding.
- Moved devices to specific module to keep the root directory clean.
- Added support for optional `PHIDGET_ROOT` to point to the directory of the phidgets22 library.
- Updated clap dependency to v4.2

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