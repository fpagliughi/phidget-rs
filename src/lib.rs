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

use phidget_sys as ffi;
use std::{ptr, ffi::CStr, os::raw::{c_char, c_uint}};

pub mod errors;
pub use crate::errors::*;

pub mod phidget;
pub use phidget::Phidget;

pub mod humidity_sensor;
pub use humidity_sensor::HumiditySensor;

pub mod temperature_sensor;
pub use temperature_sensor::TemperatureSensor;

/////////////////////////////////////////////////////////////////////////////

pub(crate) fn check_ret(rc: u32) -> Result<()> {
    match rc {
        0 => Ok(()),
        _ => Err(ReturnCode::from(rc).into()),
    }
}

pub(crate) fn get_ffi_string<F>(f: F) -> Result<String>
where
    F: Fn(*mut *const c_char) -> c_uint,
{
    unsafe {
        let mut ver: *const c_char = ptr::null_mut();
        check_ret(f(&mut ver))?;
        if ver.is_null() {
            return Err(ReturnCode::NoMemory.into());
        }
        let s = CStr::from_ptr(ver);
        Ok(s.to_string_lossy().into())
    }
}

/////////////////////////////////////////////////////////////////////////////

pub fn library_version() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersion(s) })
}

pub fn library_version_number() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersionNumber(s) })
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
