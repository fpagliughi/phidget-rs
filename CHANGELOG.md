# Change Log for phidget-rs library crate


## [v0.1.2]  - unreleased

- Added stepper implementation. Still missing async wrapper.
- Seperated voltage-io to voltage-input and ouput. (Allows better implementation of available functions from binding)
- Seperated digital-io to digital-input and ouput. (Allows better implementation of available functions from binding)
- Completion of available functions from in digital-input and ouput binding.
- Removed clap as it is not necessary to have it to use the code and it adds bloat for the examples which should be the minimum code required to use the crate.
- Moved devices to specific modul to keep the root directory clean.
- Renamed build.rs env variable to the directory of phidgets22, as well as adding a search to allow rust to find what it needs.

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
