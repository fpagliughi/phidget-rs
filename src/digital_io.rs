// phidget-rs/src/digital_io.rs
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
    self as ffi, PhidgetDigitalInputHandle as DigitalInputHandle,
    PhidgetDigitalOutputHandle as DigitalOutputHandle, PhidgetHandle,
};
use std::{
    mem,
    os::raw::{c_int, c_void},
    ptr,
};

/// The function signature for the safe Rust digital input state change callback.
pub type DigitalInputCallback = dyn Fn(&DigitalInput, i32) + Send + 'static;

/////////////////////////////////////////////////////////////////////////////

/// Phidget digital input
pub struct DigitalInput {
    // Handle to the digital input in the phidget22 library
    chan: DigitalInputHandle,
    // Double-boxed DigitalInputCallback, if registered
    cb: Option<*mut c_void>,
}

impl DigitalInput {
    /// Create a new digital input.
    pub fn new() -> Self {
        let mut chan: DigitalInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetDigitalInput_create(&mut chan);
        }
        Self { chan, cb: None }
    }

    // Low-level, unsafe, callback for the digital input state change event.
    // The context is a double-boxed pointer to the safe Rust callback.
    unsafe extern "C" fn on_state_change(chan: DigitalInputHandle, ctx: *mut c_void, state: c_int) {
        if !ctx.is_null() {
            let cb: &mut Box<DigitalInputCallback> = &mut *(ctx as *mut _);
            let sensor = Self { chan, cb: None };
            cb(&sensor, state as i32);
            mem::forget(sensor);
        }
    }

    // Drop/delete the digital input state change callback.
    // This deletes the heap memory used by the callback lambda. It must not
    // be done if the callback is still running.
    unsafe fn drop_callback(&mut self) {
        if let Some(ctx) = self.cb.take() {
            let _: Box<Box<DigitalInputCallback>> = Box::from_raw(ctx as *mut _);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &DigitalInputHandle {
        &self.chan
    }

    /// Get the state of the digital input channel
    pub fn state(&self) -> Result<i32> {
        let mut state: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getState(self.chan, &mut state) })?;
        Ok(state as i32)
    }

    /// Sets a handler to receive digital input state change callbacks.
    pub fn set_on_state_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&DigitalInput, i32) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<DigitalInputCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalInput_setOnStateChangeHandler(
                self.chan,
                Some(Self::on_state_change),
                ctx,
            )
        })
    }

    /// Removes the digital input state change callback.
    pub fn remove_on_state_change_handler(&mut self) -> Result<()> {
        // Remove the callback
        unsafe {
            let ret = ReturnCode::result(ffi::PhidgetDigitalInput_setOnStateChangeHandler(
                self.chan,
                None,
                ptr::null_mut(),
            ));
            self.drop_callback();
            ret
        }
    }
}

impl Phidget for DigitalInput {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Default for DigitalInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe {
            ffi::PhidgetDigitalInput_delete(&mut self.chan);
            self.drop_callback();
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Phidget digital output
pub struct DigitalOutput {
    // Handle to the digital output in the phidget22 library
    chan: DigitalOutputHandle,
}

impl DigitalOutput {
    /// Create a new digital input.
    pub fn new() -> Self {
        let mut chan: DigitalOutputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetDigitalOutput_create(&mut chan);
        }
        Self { chan }
    }

    /// Get the state of the digital output channel
    pub fn state(&self) -> Result<i32> {
        let mut state: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_getState(self.chan, &mut state) })?;
        Ok(state as i32)
    }

    /// Set the state of the digital output
    /// This overrides any duty cycle that was previously set.
    pub fn set_state(&self, state: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_setState(self.chan, state as c_int) })
    }

    /// Get the duty cycle of the digital output channel
    /// This is the fraction of the time the output is high. A value of 1.0
    /// means constantly high; 0.0 means constantly low
    pub fn duty_cycle(&self) -> Result<f64> {
        let mut dc: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_getDutyCycle(self.chan, &mut dc) })?;
        Ok(dc)
    }

    /// Set the duty cycle of the digital output
    /// This is the fraction of the time the output is high. A value of 1.0
    /// means constantly high; 0.0 means constantly low
    pub fn set_duty_cycle(&self, dc: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_setDutyCycle(self.chan, dc) })
    }
}

impl Phidget for DigitalOutput {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Default for DigitalOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        unsafe {
            ffi::PhidgetDigitalOutput_delete(&mut self.chan);
            //self.drop_callback();
        }
    }
}
