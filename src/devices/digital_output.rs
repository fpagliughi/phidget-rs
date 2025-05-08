// phidget-rs/src/digital_output.rs
//
// Copyright (c) 2023-2024, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Digital Output channels.

use crate::{Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetDigitalOutputHandle, PhidgetHandle};
use std::{
    ffi::{c_int, c_void},
    mem, ptr,
};

/// The function type for the safe Rust temperature sensor attach callback.
pub type AttachCallback = dyn Fn(&mut DigitalOutput) + Send + 'static;

/// The function type for the safe Rust temperature sensor detach callback.
pub type DetachCallback = dyn Fn(&mut DigitalOutput) + Send + 'static;

/// Phidget digital output
pub struct DigitalOutput {
    // Handle to the digital output in the phidget22 library
    chan: PhidgetDigitalOutputHandle,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl DigitalOutput {
    /// Create a new digital input.
    pub fn new() -> Self {
        let mut chan: PhidgetDigitalOutputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetDigitalOutput_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe callback for device attach events
    unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetDigitalOutputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe callback for device detach events
    unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetDigitalOutputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetDigitalOutputHandle {
        &self.chan
    }

    /// Set enable failsafe
    pub fn set_enable_failsafe(&self, failsafe_time: u32) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_enableFailsafe(self.chan, failsafe_time)
        })?;
        Ok(())
    }
    /// Set reset failsafe
    pub fn set_reset_failsafe(&self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_resetFailsafe(self.chan) })?;
        Ok(())
    }

    /// Set the duty cycle of the digital output
    /// This is the fraction of the time the output is high. A value of 1.0
    /// means constantly high; 0.0 means constantly low
    pub fn set_duty_cycle(&self, duty_cycle: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_setDutyCycle(self.chan, duty_cycle)
        })?;
        Ok(())
    }

    /// Get duty cycle
    pub fn duty_cycle(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getDutyCycle(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get minimum duty cycle
    pub fn min_duty_cycle(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMinDutyCycle(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum duty cycle
    pub fn max_duty_cycle(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMaxDutyCycle(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get minimum failsafe time
    pub fn min_failsafe_time(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMinFailsafeTime(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum failsafe time
    pub fn max_failsafe_time(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMaxFailsafeTime(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set frequency
    pub fn set_frequency(&self, frequency: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_setFrequency(self.chan, frequency)
        })?;
        Ok(())
    }

    /// Get frequency
    pub fn frequency(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getFrequency(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get minimum frequency
    pub fn min_frequency(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMinFrequency(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum frequency
    pub fn max_frequency(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMaxFrequency(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set led current limit
    pub fn set_led_current_limit(&self, led_current_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_setLEDCurrentLimit(self.chan, led_current_limit)
        })?;
        Ok(())
    }

    /// Get led current limit
    pub fn led_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getLEDCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get minimum led current limit
    pub fn min_led_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMinLEDCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum led current limit
    pub fn max_led_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getMaxLEDCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get led forward voltage
    pub fn led_forward_voltage(&self) -> Result<u32> {
        let mut value: ffi::PhidgetDigitalOutput_LEDForwardVoltage = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalOutput_getLEDForwardVoltage(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set the state of the digital output
    /// This overrides any duty cycle that was previously set.
    pub fn set_state(&self, state: u8) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_setState(self.chan, state as c_int) })
    }

    /// Get the state of the digital output channel
    pub fn state(&self) -> Result<u8> {
        let mut value = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalOutput_getState(self.chan, &mut value) })?;
        Ok(value as u8)
    }

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&mut DigitalOutput) + Send + 'static,
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
        F: Fn(&mut DigitalOutput) + Send + 'static,
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

impl Phidget for DigitalOutput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for DigitalOutput {}

impl Default for DigitalOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetDigitalOutputHandle> for DigitalOutput {
    fn from(chan: PhidgetDigitalOutputHandle) -> Self {
        Self {
            chan,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetDigitalOutput_delete(&mut self.chan);
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
