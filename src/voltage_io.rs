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

use crate::{Phidget, Result, ReturnCode};
use phidget_sys::{
    self as ffi, PhidgetHandle, PhidgetVoltageInputHandle as VoltageInputHandle,
    PhidgetVoltageOutputHandle as VoltageOutputHandle,
};
use std::{mem, os::raw::c_void, ptr};

/// The function signature for the safe Rust voltage change callback.
pub type VoltageChangeCallback = dyn Fn(&VoltageInput, f64) + Send + 'static;

/////////////////////////////////////////////////////////////////////////////

/// Phidget voltage input
pub struct VoltageInput {
    // Handle to the voltage input in the phidget22 library
    chan: VoltageInputHandle,
    // Double-boxed VoltageChangeCallback, if registered
    cb: Option<*mut c_void>,
}

impl VoltageInput {
    /// Create a new voltage input.
    pub fn new() -> Self {
        let mut chan: VoltageInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetVoltageInput_create(&mut chan);
        }
        Self { chan, cb: None }
    }

    // Low-level, unsafe, callback for the voltage change event.
    // The context is a double-boxed pointer to the safe Rust callback.
    unsafe extern "C" fn on_voltage_change(
        chan: VoltageInputHandle,
        ctx: *mut c_void,
        voltage: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<VoltageChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self { chan, cb: None };
            cb(&sensor, voltage);
            mem::forget(sensor);
        }
    }

    // Drop/delete the voltage change callback.
    // This deletes the heap memory used by the callback lambda. It must not
    // be done if the callback is still running.
    unsafe fn drop_callback(&mut self) {
        if let Some(ctx) = self.cb.take() {
            let _: Box<Box<VoltageChangeCallback>> = unsafe { Box::from_raw(ctx as *mut _) };
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &VoltageInputHandle {
        &self.chan
    }

    /// Get the voltage on the input channel
    pub fn voltage(&self) -> Result<f64> {
        let mut v: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetVoltageInput_getVoltage(self.chan, &mut v) })?;
        Ok(v)
    }

    /// Sets a handler to receive voltage change callbacks.
    pub fn set_on_voltage_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&VoltageInput, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<VoltageChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetVoltageInput_setOnVoltageChangeHandler(
                self.chan,
                Some(Self::on_voltage_change),
                ctx,
            )
        })
    }

    /// Removes the voltage change callback.
    pub fn remove_on_voltage_change_handler(&mut self) -> Result<()> {
        // Remove the callback
        unsafe {
            let ret = ReturnCode::result(ffi::PhidgetVoltageInput_setOnVoltageChangeHandler(
                self.chan,
                None,
                ptr::null_mut(),
            ));
            self.drop_callback();
            ret
        }
    }
}

impl Phidget for VoltageInput {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Default for VoltageInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VoltageInput {
    fn drop(&mut self) {
        unsafe {
            ffi::PhidgetVoltageInput_delete(&mut self.chan);
            self.drop_callback();
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Phidget voltage output
pub struct VoltageOutput {
    // Handle to the voltage output in the phidget22 library
    chan: VoltageOutputHandle,
}

impl VoltageOutput {
    /// Create a new voltage input.
    pub fn new() -> Self {
        let mut chan: VoltageOutputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetVoltageOutput_create(&mut chan);
        }
        Self { chan }
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
}

impl Phidget for VoltageOutput {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Default for VoltageOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VoltageOutput {
    fn drop(&mut self) {
        unsafe {
            ffi::PhidgetVoltageOutput_delete(&mut self.chan);
        }
    }
}
