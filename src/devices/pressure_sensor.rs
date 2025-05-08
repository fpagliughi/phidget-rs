// phidget-rs/src/pressure_sensor.rs
//
// Copyright (c) 2023-2025, Frank Pagliughi
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
    self as ffi, PhidgetHandle, PhidgetPressureSensorHandle as PressureSensorHandle,
};
use std::{ffi::c_void, mem, ptr};

/////////////////////////////////////////////////////////////////////////////

/// The function type for the safe Rust pressure sensor attach callback.
pub type AttachCallback = dyn Fn(&mut PressureSensor) + Send + 'static;

/// The function type for the safe Rust pressure sensor detach callback.
pub type DetachCallback = dyn Fn(&mut PressureSensor) + Send + 'static;

/// The function type for the safe Rust pressure change callback.
pub type PressureChangeCallback = dyn Fn(&PressureSensor, f64) + Send + 'static;

/// Phidget pressure sensor
pub struct PressureSensor {
    // Handle to the sensor for the phidget22 library
    chan: PressureSensorHandle,
    // Double-boxed PressureChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl PressureSensor {
    /// Create a new pressure sensor.
    pub fn new() -> Self {
        let mut chan: PressureSensorHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetPressureSensor_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe callback for device attach events
    unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PressureSensorHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe callback for device detach events
    unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PressureSensorHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe, callback for pressure change events.
    // The context is a double-boxed pointer the the safe Rust callback.
    unsafe extern "C" fn on_pressure_change(
        chan: PressureSensorHandle,
        ctx: *mut c_void,
        pressure: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<PressureChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, pressure);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PressureSensorHandle {
        &self.chan
    }

    /// Read the current pressure
    pub fn pressure(&self) -> Result<f64> {
        let mut pressure = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getPressure(self.chan, &mut pressure)
        })?;
        Ok(pressure)
    }

    /// Gets the minimum value the `PressureChange` event will report.
    pub fn min_pressure(&self) -> Result<f64> {
        let mut pressure = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getMinPressure(self.chan, &mut pressure)
        })?;
        Ok(pressure)
    }

    /// Gets the maximum value the `PressureChange` event will report.
    pub fn max_pressure(&self) -> Result<f64> {
        let mut pressure = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getMaxPressure(self.chan, &mut pressure)
        })?;
        Ok(pressure)
    }

    /// Gets the current value of the `PressureChangeTrigger`.
    ///
    /// The channel will not issue a PressureChange event until the
    /// pressure value has changed by the amount specified by the
    /// PressureChangeTrigger.
    pub fn pressure_change_trigger(&self) -> Result<f64> {
        let mut pressure = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getPressureChangeTrigger(self.chan, &mut pressure)
        })?;
        Ok(pressure)
    }

    /// Sets the `PressureChangeTrigger`.
    ///
    /// The channel will not issue a PressureChange event until the
    /// pressure value has changed by the amount specified by the
    /// PressureChangeTrigger.
    /// Gets the maximum value the `PressureChange` event will report.
    ///
    /// Setting this to 0 will result in the channel firing events every
    /// DataInterval. This is useful for applications that implement their
    /// own data filtering.
    pub fn set_pressure_change_trigger(&self, trigger: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_setPressureChangeTrigger(self.chan, trigger)
        })?;
        Ok(())
    }

    /// Gets the minimum value of the `PressureChangeTrigger`.
    pub fn min_pressure_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getMinPressureChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Gets the maximum value of the `PressureChangeTrigger`.
    pub fn max_pressure_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_getMaxPressureChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Set a handler to receive pressure change callbacks.
    pub fn set_on_pressure_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&PressureSensor, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<PressureChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetPressureSensor_setOnPressureChangeHandler(
                self.chan,
                Some(Self::on_pressure_change),
                ctx,
            )
        })
    }

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&mut PressureSensor) + Send + 'static,
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
        F: Fn(&mut PressureSensor) + Send + 'static,
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

impl Phidget for PressureSensor {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for PressureSensor {}

impl Default for PressureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PressureSensorHandle> for PressureSensor {
    fn from(chan: PressureSensorHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for PressureSensor {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetPressureSensor_delete(&mut self.chan);
            crate::drop_cb::<PressureChangeCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
