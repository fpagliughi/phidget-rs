# phidget-rs

A safe Rust wrapper around the `phidgets22` library for interacting with [Phidget sensors and actuators](https://www.phidgets.com/).

This is currently an early-stage project to wrap the Phidget API. It is intended to be a production-quality crate for use in real industrial settings.

Note that the authors only have a limited number of sensors available. Pull Requests are glady accepted for any additional types of Phidgets to add to the library, or let us know if you have a device that you would like to see supported and are willing to test and validate.

## Minimum Supported Rust Version (MSRV)

**v1.73**

This package uses Rust Edition 2021, requiring an MSRV of 1.73. Although it may build and work with slightly older versions of the compiler, this is the oldest version being tested and maintained by the developers.
