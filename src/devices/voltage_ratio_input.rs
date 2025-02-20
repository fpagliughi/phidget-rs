// phidget-rs/examples/voltage_in_ratio.rs
//
// Copyright (c) 2023, Frank Pagliughi
// Copyright (c) 2024 Jorge Guerra and Riley Hernandez
//
// This file is an example application for the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//
use crate::{AttachCallback, DetachCallback, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetVoltageRatioInputHandle};
use std::{mem, os::raw::c_void, ptr};

/// The function type for the safe Rust position change callback.
pub type VoltageRatioChangeCallback = dyn Fn(&VoltageRatioInput, f64) + Send + 'static;

/// Phidget voltage ratio input.
pub struct VoltageRatioInput {
    // Handle to the voltage ratio input in the phidget22 library
    chan: PhidgetVoltageRatioInputHandle,
    // Double-boxed VoltageRatioChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl VoltageRatioInput {
    /// Create a new voltage ratio input.
    pub fn new() -> Self {
        let mut chan: PhidgetVoltageRatioInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetVoltageRatioInput_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe, callback for the voltage ratio change event.
    // The context is a double-boxed pointer to the safe Rust callback.
    unsafe extern "C" fn on_voltage_ratio_change(
        chan: PhidgetVoltageRatioInputHandle,
        ctx: *mut c_void,
        voltage: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<VoltageRatioChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, voltage);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetVoltageRatioInputHandle {
        &self.chan
    }

    /// Get the voltage ratio on the input channel
    pub fn voltage_ratio(&self) -> Result<f64> {
        let mut voltage_ratio: f64 = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetVoltageRatioInput_getVoltageRatio(self.chan, &mut voltage_ratio)
        })?;
        Ok(voltage_ratio)
    }

    /// Sets a handler to receive voltage change callbacks.
    pub fn set_on_voltage_ratio_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&VoltageRatioInput, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<VoltageRatioChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetVoltageRatioInput_setOnVoltageRatioChangeHandler(
                self.chan,
                Some(Self::on_voltage_ratio_change),
                ctx,
            )
        })
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

impl Phidget for VoltageRatioInput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for VoltageRatioInput {}

impl Default for VoltageRatioInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetVoltageRatioInputHandle> for VoltageRatioInput {
    fn from(chan: PhidgetVoltageRatioInputHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for VoltageRatioInput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetVoltageRatioInput_delete(&mut self.chan);
            crate::drop_cb::<VoltageRatioChangeCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
