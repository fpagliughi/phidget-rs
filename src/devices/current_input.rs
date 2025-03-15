// phidget-rs/src/current_input.rs
//
// Copyright (c) 2025, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::{AttachCallback, DetachCallback, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetCurrentInputHandle as CurrentInputHandle, PhidgetHandle};
use std::{mem, os::raw::c_void, ptr};

/////////////////////////////////////////////////////////////////////////////

/// The function type for the safe Rust current_input change callback.
pub type CurrentChangeCallback = dyn Fn(&CurrentInput, f64) + Send + 'static;

/// Phidget current input sensor.
pub struct CurrentInput {
    // Handle to the sensor for the phidget22 library
    chan: CurrentInputHandle,
    // Double-boxed CurrentChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl CurrentInput {
    /// Create a new current_input sensor.
    pub fn new() -> Self {
        let mut chan: CurrentInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetCurrentInput_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe, callback for current_input change events.
    // The context is a double-boxed pointer the the safe Rust callback.
    unsafe extern "C" fn on_current_change(
        chan: CurrentInputHandle,
        ctx: *mut c_void,
        current_input: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<CurrentChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, current_input);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &CurrentInputHandle {
        &self.chan
    }

    /// Read the current.
    pub fn current(&self) -> Result<f64> {
        let mut current = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getCurrent(self.chan, &mut current)
        })?;
        Ok(current)
    }

    /// Gets the minimum value the `CurrentChange` event will report.
    pub fn min_current(&self) -> Result<f64> {
        let mut current = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getMinCurrent(self.chan, &mut current)
        })?;
        Ok(current)
    }

    /// Gets the maximum value the `CurrentChange` event will report.
    pub fn max_current(&self) -> Result<f64> {
        let mut current = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getMaxCurrent(self.chan, &mut current)
        })?;
        Ok(current)
    }

    /// Gets the current value of the `CurrentChangeTrigger`.
    ///
    /// The channel will not issue a CurrentChange event until the
    /// current value has changed by the amount specified by the
    /// CurrentChangeTrigger.
    pub fn current_change_trigger(&self) -> Result<f64> {
        let mut current = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getCurrentChangeTrigger(self.chan, &mut current)
        })?;
        Ok(current)
    }

    /// Sets the `CurrentChangeTrigger`.
    ///
    /// The channel will not issue a CurrentChange event until the
    /// current value has changed by the amount specified by the
    /// CurrentChangeTrigger.
    /// Gets the maximum value the `CurrentChange` event will report.
    ///
    /// Setting this to 0 will result in the channel firing events every
    /// DataInterval. This is useful for applications that implement their
    /// own data filtering.
    pub fn set_current_change_trigger(&self, trigger: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_setCurrentChangeTrigger(self.chan, trigger)
        })?;
        Ok(())
    }

    /// Gets the minimum value of the `CurrentChangeTrigger`.
    pub fn min_current_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getMinCurrentChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Gets the maximum value of the `CurrentChangeTrigger`.
    pub fn max_current_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_getMaxCurrentChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Set a handler to receive current change callbacks.
    pub fn set_on_current_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&CurrentInput, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<CurrentChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetCurrentInput_setOnCurrentChangeHandler(
                self.chan,
                Some(Self::on_current_change),
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

impl Phidget for CurrentInput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for CurrentInput {}

impl Default for CurrentInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CurrentInputHandle> for CurrentInput {
    fn from(chan: CurrentInputHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for CurrentInput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetCurrentInput_delete(&mut self.chan);
            crate::drop_cb::<CurrentChangeCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
