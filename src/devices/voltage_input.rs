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
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetVoltageInputHandle};
use std::{ffi::c_void, mem, ptr};

/// The function signature for the safe Rust voltage change callback.
pub type VoltageChangeCallback = dyn Fn(&VoltageInput, f64) + Send + 'static;

/////////////////////////////////////////////////////////////////////////////

/// Phidget voltage input
pub struct VoltageInput {
    // Handle to the voltage input in the phidget22 library
    chan: PhidgetVoltageInputHandle,
    // Double-boxed VoltageChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl VoltageInput {
    /// Create a new voltage input.
    pub fn new() -> Self {
        let mut chan: PhidgetVoltageInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetVoltageInput_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe, callback for the voltage change event.
    // The context is a double-boxed pointer to the safe Rust callback.
    unsafe extern "C" fn on_voltage_change(
        chan: PhidgetVoltageInputHandle,
        ctx: *mut c_void,
        voltage: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<VoltageChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, voltage);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetVoltageInputHandle {
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

impl Phidget for VoltageInput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for VoltageInput {}

impl Default for VoltageInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetVoltageInputHandle> for VoltageInput {
    fn from(chan: PhidgetVoltageInputHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for VoltageInput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetVoltageInput_delete(&mut self.chan);
            crate::drop_cb::<VoltageChangeCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
