# Change Log for phidget-rs library crate

## [v0.4.0](https://github.com/fpagliughi/phidget-rs/compare/v0.3.1..v0.4.0)  - 2024-05-08

- Attach & Detach callbacks are now device-specific for all the devices
    - Callbacks get a mutable reference to the specific device, not to a `PhidgetRef`
- Added support for Pressure Sensors
- Added examples for digital and voltage (analog) outputs
- Fixed serde deserialization of `PhidgetInfo`
- `PhidgetInfo::device_label` changed to a `String` from `Option<String>`
- Bumped -sys crate's MSRV to v1.73.0
- Bumped `clap` dependency to v4.4 for utils and examples
- [#14](https://github.com/fpagliughi/phidget-rs/pull/14) Added thermocouple-type parameter to temperature sensor example.

## [v0.3.1](https://github.com/fpagliughi/phidget-rs/compare/v0.3.0..v0.3.1)  - 2024-05-05

- Added `TryFrom<&PhidgetRef>` for `GenericPhidget`
- Re-exported `GenericPhidget` to the top-level module.


## [v0.3.0](https://github.com/fpagliughi/phidget-rs/compare/v0.2.0..v0.3.0)  - 2025-04-30

- [Breaking] Changes to the `Phidget` trait and non-owning containers.
    - `GenericPhidget` is now an owning wrapper.
    - `PhidgetRef` is now the non-owning reference to a generic `PhidgetHandle`
    - Callbacks now get a `PhidgetRef`, which is also `!Send` and `!Sync`. Thus, they can not "escape" from the callback thread.
    - The `GenericPhidget` can currently only be created from a `PhidgetRef`, and implements `Send`.
        - It manages the lifetime of the handle with `ffi::Phidget_retain()` and `ffi::Phidget_release()`.
    - To keep a Phidget from a hotplug add event, convert the `PhidgetRef` to a `GenericPhidget` and send it out of the callback.
- The `Phidget` trait no longer requires the `Send` trait, though most device types should manually implement `Send` to be useful.
- The `Phidget` trait supports reading and writing device labels to Flash
- Added DeviceId type and Phidget::device_id() query
- Themperature senor has getters and setters for `RtdType`, `RtdWireSetup`, and `ThermocoupleType`.
- Added _serde_ build feature to support serializing data type like `PhidgetInfo`, etc
- Base types moved to a new _types_ module
- The -sys crate uses the default installer location for phidget22.lib on Windows by default.
- [#13](https://github.com/fpagliughi/phidget-rs/pull/13) Implemented Fidget Manager
    - The `Phidget` trait now requires: `fn as_mut_handle(&mut self) -> PhidgetHandle;`
- Added an example for hotplug events
- The CLI _utils_ build feature no longer enabled by default.

- The `-sys` crate:
    - Uses the default install location for the `phidget22.lib` library on Windows, `C:\Program Files\Phidgets\Phidget22`.
        - No longer a need to set the `PHIDGET_ROOT` environment variable for the default install.


## [v0.2.0](https://github.com/fpagliughi/phidget-rs/compare/v0.1.4..v0.2.0)  - 2024-10-21

- Digital In & Out state consistently represented with a u8
- [#11](https://github.com/fpagliughi/phidget-rs/pull/1) Examples in the documentation.
- Minor updates and output formatting to the example apps
- Created a config file for the Rust 'typos' utility, and used it to run a spell check.


## [v0.1.4](https://github.com/fpagliughi/phidget-rs/compare/v0.1.3..v0.1.4)  - 2024-05-30

- [#8](https://github.com/fpagliughi/phidget-rs/pull/8) Add voltage ratio input
- Added 32-bit bindings in -sys crate.
- Added stepper motor with example. Missing test and async functionality


## [v0.1.3](https://github.com/fpagliughi/phidget-rs/compare/v0.1.2..v0.1.3)  - 2024-03-10

- Exporting devices (`DigitalInput`, `DigitalOutput`, etc) at the crate root to fix accidental breaking change in v0.1.x


## [v0.1.2](https://github.com/fpagliughi/phidget-rs/compare/v0.1.1..v0.1.2)  - 2024-03-10

- Bumped MSRV to 1.73
- Separated voltage-io to voltage-input and output modules.
- Separated digital-io to digital-input and output modules.
- Completion of available functions from in digital-input and output binding.
- Moved devices to specific module to keep the root directory clean.
- Added support for optional `PHIDGET_ROOT` to point to the directory of the phidgets22 library.


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
