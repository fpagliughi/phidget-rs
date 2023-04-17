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

// Platform dependent whether necessary
#![allow(clippy::unnecessary_cast)]

use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_uint},
    ptr,
};

pub use phidget_sys::{
    self as ffi, PHIDGET_CHANNEL_ANY, PHIDGET_HUBPORTSPEED_AUTO, PHIDGET_HUBPORT_ANY,
    PHIDGET_SERIALNUMBER_ANY, PHIDGET_TIMEOUT_DEFAULT, PHIDGET_TIMEOUT_INFINITE,
};

pub mod errors;
pub use crate::errors::*;

pub mod phidget;
pub use crate::phidget::Phidget;

pub mod humidity_sensor;
pub use crate::humidity_sensor::HumiditySensor;

pub mod temperature_sensor;
pub use crate::temperature_sensor::TemperatureSensor;

/////////////////////////////////////////////////////////////////////////////

/// Gets a string from a phidget22 call.
/// This can be any function that takes a pointer to a c-str as the lone
/// argument.
pub(crate) fn get_ffi_string<F>(f: F) -> Result<String>
where
    F: Fn(*mut *const c_char) -> c_uint,
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

/// Gets an integer from a phidget22 call.
/// This can be any function that takes a pointer to a C `int` as the lone
/// argument.
pub(crate) fn get_ffi_int<F>(mut f: F) -> Result<i32>
where
    F: FnMut(*mut c_int) -> c_uint,
{
    let mut n: c_int = 0;
    ReturnCode::result(f(&mut n))?;
    Ok(n as i32)
}

/////////////////////////////////////////////////////////////////////////////

/// The the full version of the phidget22 library as a string.
/// This is somthing like, "Phidget22 - Version 1.14 - Built Mar 31 2023 22:44:59"
pub fn library_version() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersion(s) })
}

/// Gets just the version number of the phidget22 library as a string.
/// This is something like, "1.14"
pub fn library_version_number() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersionNumber(s) })
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
