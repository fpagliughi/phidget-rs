// phidget-rs/src/lib.rs
//
// Copyright (c) 2023, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Safe Rust bindings to the phidget22 library.
//!
//! # Basic usage
//!
//! This example shows how to access a simple Digital Input, connected to the first available channel of a Vint HUB.
//! See the `examples` directory for more thorough code snippets.
//! ```rust,no_run
//! use phidget::{DigitalOutput, Phidget};
//! # use std::time::Duration;
//!
//! // Create a handle to a Digital Output device
//! let mut out = DigitalOutput::new();
//! // Before opening the device, set its VINT hub port
//! out.set_is_hub_port_device(true).unwrap();
//! out.set_hub_port(0).unwrap();
//!
//! // Start connection. Make sure to handle the result
//! // to check the device is available
//! out.open_wait_default().unwrap();
//!
//! // Control the output device
//! loop {
//!     println!("Turn on LED");
//!     out.set_state(1).unwrap();
//!     std::thread::sleep(Duration::from_secs(3));
//!
//!     println!("Turn off LED");
//!     out.set_state(0).unwrap();
//!     std::thread::sleep(Duration::from_secs(3));
//! }
//! ```
//!
//! # Callbacks
//! In order to activate an output phidget device depending on the state of other sensors,
//! for instance by turning on an LED whenever another sensor detects something,
//! you need to set a callback listening for sensor value changes, and keep a valid handle to the output device to set its state.
//!
//! The problem is, Phidget callbacks do run in a different thread. A Phidget handle can already be sent
//! to a different thread, as it implements [Send], but it doesn't implement [Sync].
//! Hence, if you desire to access the same handle from different callbacks, it has to be wrapped in a
//! Sync container, such as a [Mutex](std::sync::Mutex).
//!
//! ```rust,no_run
//! # use phidget::{Phidget, DigitalOutput, DigitalInput};
//! # use std::sync::Mutex;
//! # fn main()
//! # {
//! #    // Open a digitalInput to detect a button
//!     let mut button = DigitalInput::new();
//! #   button.set_channel(0).unwrap();
//!     // Open the digital output where
//!     // a LED is connected to.
//!     // In this example, it is initialized
//!     // and wrapped in a Mutex
//!     let led = Mutex::new({
//!         let mut tmp = DigitalOutput::new();
//!         tmp.set_channel(1).unwrap();
//!         tmp.open_wait_default().unwrap();
//!         tmp
//!     });
//!
//!     // Make the button alternate the LED state
//!     button.set_on_state_change_handler(move |_, s: u8| {
//!         let lock = led.lock().unwrap();
//!         match s {
//!             // Access the device inside the Mutex and change its state
//!             0 => lock.set_state(0).unwrap(),
//!             _ => lock.set_state(1).unwrap()
//!         }
//!     }).unwrap();
//! # }
//! ```

// Platform dependent whether necessary
#![allow(clippy::unnecessary_cast)]
// Lints
#![deny(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::{
    ffi::CStr,
    os::raw::{c_char, c_uint, c_void},
    ptr,
    time::Duration,
};

pub use phidget_sys::{
    self as ffi, PHIDGET_CHANNEL_ANY, PHIDGET_HUBPORTSPEED_AUTO, PHIDGET_HUBPORT_ANY,
    PHIDGET_SERIALNUMBER_ANY, PHIDGET_TIMEOUT_DEFAULT, PHIDGET_TIMEOUT_INFINITE,
};

/// The error types for the crate
pub mod errors;
pub use crate::errors::*;

/// The enumerated types for the crate
pub mod types;
pub use crate::types::*;

/// The main Phidget trait
pub mod phidget;
pub use crate::phidget::{AttachCallback, DetachCallback, Phidget, PhidgetRef};

/// Network API
pub mod net;
pub use crate::net::ServerType;

/// Module containing all implemented devices
pub mod devices;
pub mod manager;

// For v0.1.x compatibility, sensors available at the root
pub use crate::devices::{
    digital_input::DigitalInput, digital_output::DigitalOutput, hub::Hub,
    humidity_sensor::HumiditySensor, temperature_sensor::TemperatureSensor,
    voltage_input::VoltageInput, voltage_output::VoltageOutput,
    voltage_ratio_input::VoltageRatioInput,
};

/// An infinite timeout (wait forever)
pub const TIMEOUT_INFINITE: Duration = Duration::from_millis(PHIDGET_TIMEOUT_INFINITE as u64);

/// The default timeout for the library
pub const TIMEOUT_DEFAULT: Duration = Duration::from_millis(PHIDGET_TIMEOUT_DEFAULT as u64);

/////////////////////////////////////////////////////////////////////////////
/// Gets a string from a phidget22 call.
/// This can be any function that takes a pointer to a c-str as the lone
/// argument.
pub(crate) fn get_ffi_string<F>(mut f: F) -> Result<String>
where
    F: FnMut(*mut *const c_char) -> c_uint,
{
    unsafe {
        let mut ver: *const c_char = ptr::null_mut();
        ReturnCode::result(f(&mut ver))?;
        if ver.is_null() {
            return Err(ReturnCode::NoMemory);
        }
        let s = CStr::from_ptr(ver);
        Ok(s.to_string_lossy().into())
    }
}

/// Release the memory held in a double-boxed callback function/lambda.
pub(crate) fn drop_cb<P: ?Sized>(cb: Option<*mut c_void>) {
    if let Some(ctx) = cb {
        let _: Box<Box<P>> = unsafe { Box::from_raw(ctx as *mut _) };
    }
}

/////////////////////////////////////////////////////////////////////////////

/// The the full version of the phidget22 library as a string.
/// This is something like, "Phidget22 - Version 1.14 - Built Mar 31 2023 22:44:59"
pub fn library_version() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersion(s) })
}

/// Gets just the version number of the phidget22 library as a string.
/// This is something like, "1.14"
pub fn library_version_number() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersionNumber(s) })
}

/// Closes all channels, and stops all threads. The library is reset to a
/// newly loaded state. All channel handles have been freed.
///
/// This function is intended for use in special cases where the library
/// cannot be unloaded between program runs, such as LabVIEW and Unity
/// Editor.
///
/// # Safety
///
/// This invalidates all Phidget objects that are running.
///
pub unsafe fn reset_library() -> Result<()> {
    ReturnCode::result(unsafe { ffi::Phidget_resetLibrary() })
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_queries() {
        assert!(library_version().is_ok());
        assert!(library_version_number().is_ok());
    }
}
