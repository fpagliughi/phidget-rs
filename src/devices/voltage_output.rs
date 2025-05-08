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
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetVoltageOutputHandle};
use std::{ffi::c_void, mem, ptr};

/////////////////////////////////////////////////////////////////////////////

/// The function type for the safe Rust voltage output attach callback.
pub type AttachCallback = dyn Fn(&mut VoltageOutput) + Send + 'static;

/// The function type for the safe Rust voltage output detach callback.
pub type DetachCallback = dyn Fn(&mut VoltageOutput) + Send + 'static;

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

    // Low-level, unsafe callback for device attach events
    unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetVoltageOutputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe callback for device detach events
    unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetVoltageOutputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetVoltageOutputHandle {
        &self.chan
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
        F: Fn(&mut VoltageOutput) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<AttachCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;

        ReturnCode::result(unsafe {
            ffi::Phidget_setOnAttachHandler(self.as_mut_handle(), Some(Self::on_attach), ctx)
        })?;
        self.attach_cb = Some(ctx);
        Ok(())
    }

    /// Sets a handler to receive detach callbacks
    pub fn set_on_detach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&mut VoltageOutput) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<DetachCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;

        ReturnCode::result(unsafe {
            ffi::Phidget_setOnDetachHandler(self.as_mut_handle(), Some(Self::on_detach), ctx)
        })?;
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
