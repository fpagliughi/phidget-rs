// phidget-rs/src/voltage_io.rs
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

use crate::{AttachCallback, DetachCallback, Phidget, PhidgetRef, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetVoltageOutputHandle};
use std::{ffi::c_void, ptr};

/// Phidget voltage output
pub struct VoltageOutput {
    // Handle to the voltage output in the phidget22 library
    chan: PhidgetVoltageOutputHandle,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl VoltageOutput {
    /// Create a new voltage input.
    pub fn new() -> Self {
        let mut chan: PhidgetVoltageOutputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetVoltageOutput_create(&mut chan);
        }
        Self::from(chan)
    }

    /// Get the voltage value that the channel will output
    pub fn voltage(&self) -> Result<f64> {
        let mut v: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetVoltageOutput_getVoltage(self.chan, &mut v) })?;
        Ok(v)
    }

    /// Set the voltage value that the channel will output.
    pub fn set_voltage(&self, v: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetVoltageOutput_setVoltage(self.chan, v) })
    }

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&PhidgetRef) + Send + 'static,
    {
        let ctx = crate::phidget::set_on_attach_handler(self, cb)?;
        self.attach_cb = Some(ctx);
        Ok(())
    }

    /// Sets a handler to receive detach callbacks
    pub fn set_on_detach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&PhidgetRef) + Send + 'static,
    {
        let ctx = crate::phidget::set_on_detach_handler(self, cb)?;
        self.detach_cb = Some(ctx);
        Ok(())
    }
}

impl Phidget for VoltageOutput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for VoltageOutput {}

impl Default for VoltageOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetVoltageOutputHandle> for VoltageOutput {
    fn from(chan: PhidgetVoltageOutputHandle) -> Self {
        Self {
            chan,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for VoltageOutput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetVoltageOutput_delete(&mut self.chan);
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
