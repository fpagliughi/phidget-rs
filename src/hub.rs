// phidget-rs/src/hub.rs
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

use crate::{Error, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetHubHandle as HubHandle};
use std::{
    os::raw::{c_int, c_uint},
    ptr,
};

/////////////////////////////////////////////////////////////////////////////

/// Possible operational modes for a hub port
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HubPortMode {
    /// Communicate with a smart VINT device
    Vint = ffi::PhidgetHub_PortMode_PORT_MODE_VINT_PORT, // 0
    /// 5V Logic-level digital input
    DigitalInput = ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_INPUT, // 1
    /// 3.3V digital output
    DigitalOutput = ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_OUTPUT, // 2
    /// 0-5V voltage input for non-ratiometric sensors
    VoltageInput = ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_INPUT, // 3
    /// 0-5V voltage input for ratiometric sensors
    VoltageRatioInput = ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_RATIO_INPUT, // 4
}

impl TryFrom<u32> for HubPortMode {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use HubPortMode::*;
        match val {
            ffi::PhidgetHub_PortMode_PORT_MODE_VINT_PORT => Ok(Vint), // 0
            ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_INPUT => Ok(DigitalInput), // 1
            ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_OUTPUT => Ok(DigitalOutput), // 2
            ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_INPUT => Ok(VoltageInput), // 3
            ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_RATIO_INPUT => Ok(VoltageRatioInput), // 4
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget Hub
pub struct Hub {
    // Handle to the hub in the phidget22 library
    chan: HubHandle,
}

impl Hub {
    /// Create a new hub.
    pub fn new() -> Self {
        let mut chan: HubHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetHub_create(&mut chan);
        }
        Self { chan }
    }

    /// Get the mode of the specified hub port
    pub fn port_mode(&self, port: i32) -> Result<HubPortMode> {
        let port = port as c_int;
        let mut mode: c_uint = 0;
        ReturnCode::result(unsafe { ffi::PhidgetHub_getPortMode(self.chan, port, &mut mode) })?;
        HubPortMode::try_from(mode)
    }

    /// Set the mode of the specified hub port
    pub fn set_port_mode(&self, port: i32, mode: HubPortMode) -> Result<()> {
        let port = port as c_int;
        ReturnCode::result(unsafe { ffi::PhidgetHub_setPortMode(self.chan, port, mode as c_uint) })
    }
}

impl Phidget for Hub {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for Hub {}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Hub {
    fn drop(&mut self) {
        unsafe {
            ffi::PhidgetHub_delete(&mut self.chan);
        }
    }
}
