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

use crate::{AttachCallback, DetachCallback, Error, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetHubHandle as HubHandle};
use std::{
    os::raw::{c_int, c_uint, c_void},
    ptr,
};

/////////////////////////////////////////////////////////////////////////////

/// Possible operational modes for a hub port
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HubPortMode {
    /// Communicate with a smart VINT device
    Vint = ffi::PhidgetHub_PortMode_PORT_MODE_VINT_PORT,
    /// 5V Logic-level digital input
    DigitalInput = ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_INPUT,
    /// 3.3V digital output
    DigitalOutput = ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_OUTPUT,
    /// 0-5V voltage input for non-ratiometric sensors
    VoltageInput = ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_INPUT,
    /// 0-5V voltage input for ratiometric sensors
    VoltageRatioInput = ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_RATIO_INPUT,
}

impl TryFrom<u32> for HubPortMode {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use HubPortMode::*;
        match val {
            ffi::PhidgetHub_PortMode_PORT_MODE_VINT_PORT => Ok(Vint),
            ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_INPUT => Ok(DigitalInput),
            ffi::PhidgetHub_PortMode_PORT_MODE_DIGITAL_OUTPUT => Ok(DigitalOutput),
            ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_INPUT => Ok(VoltageInput),
            ffi::PhidgetHub_PortMode_PORT_MODE_VOLTAGE_RATIO_INPUT => Ok(VoltageRatioInput),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget Hub
pub struct Hub {
    // Handle to the hub in the phidget22 library
    chan: HubHandle,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl Hub {
    /// Create a new hub.
    pub fn new() -> Self {
        let mut chan: HubHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetHub_create(&mut chan);
        }
        Self::from(chan)
    }

    /// Get the mode of the specified hub port
    pub fn port_mode(&self, port: i32) -> Result<HubPortMode> {
        let mut mode: c_uint = 0;
        ReturnCode::result(unsafe { ffi::PhidgetHub_getPortMode(self.chan, port, &mut mode) })?;
        HubPortMode::try_from(mode)
    }

    /// Set the mode of the specified hub port
    pub fn set_port_mode(&self, port: i32, mode: HubPortMode) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetHub_setPortMode(self.chan, port, mode as c_uint) })
    }

    /// Determines if power is enabled for this VINT port
    pub fn is_port_power_enabled(&self, port: i32) -> Result<bool> {
        let mut state: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetHub_getPortPower(self.chan, port, &mut state) })?;
        Ok(state != 0)
    }

    /// Enables/disables power to the VINT hub port.
    pub fn enable_port_power(&mut self, port: i32, on: bool) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetHub_setPortPower(self.chan, port, on as c_int)
        })
    }

    /// Enables/disables Auto Set Speed on the hub port.
    ///
    /// When enabled, and a supported VINT device is attached, the
    /// `HubPortSpeed` will automatically be set to the fastest reliable
    /// speed. This is enabled by default on supported VINT Hubs.
    pub fn enable_port_auto_set_speed(&mut self, port: i32, on: bool) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetHub_setPortAutoSetSpeed(self.chan, port, on as c_int)
        })
    }

    /// Determines if this VINT port supports Auto Set Speed
    pub fn port_supports_auto_set_speed(&self, port: i32) -> Result<bool> {
        let mut state: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetHub_getPortSupportsAutoSetSpeed(self.chan, port, &mut state) })?;
        Ok(state != 0)
    }

    /// Determines if the communication speed of this VINT port can be set.
    pub fn port_supports_set_speed(&self, port: i32) -> Result<bool> {
        let mut state: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetHub_getPortSupportsSetSpeed(self.chan, port, &mut state) })?;
        Ok(state != 0)
    }

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&GenericPhidget) + Send + 'static,
    {
        let ctx = crate::phidget::set_on_attach_handler(self, cb)?;
        self.attach_cb = Some(ctx);
        Ok(())
    }

    /// Sets a handler to receive detach callbacks
    pub fn set_on_detach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&GenericPhidget) + Send + 'static,
    {
        let ctx = crate::phidget::set_on_detach_handler(self, cb)?;
        self.detach_cb = Some(ctx);
        Ok(())
    }
}

impl Phidget for Hub {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for Hub {}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HubHandle> for Hub {
    fn from(chan: HubHandle) -> Self {
        Self {
            chan,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for Hub {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetHub_delete(&mut self.chan);
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
