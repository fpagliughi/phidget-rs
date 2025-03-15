// phidget-rs/src/humidity_sensor.rs
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
//! Phidget Humidity sensor
//!

use crate::{AttachCallback, DetachCallback, Phidget, PhidgetRef, Result, ReturnCode};
use phidget_sys::{
    self as ffi, PhidgetHandle, PhidgetHumiditySensorHandle as HumiditySensorHandle,
};
use std::{mem, os::raw::c_void, ptr};

/// The function signature for the safe Rust humidity change callback.
pub type HumidityChangeCallback = dyn Fn(&HumiditySensor, f64) + Send + 'static;

/// Phidget humidity sensor
pub struct HumiditySensor {
    // Handle to the sensor for the phidget22 library
    chan: HumiditySensorHandle,
    // Double-boxed HumidityChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl HumiditySensor {
    /// Create a new humidity sensor.
    pub fn new() -> Self {
        let mut chan: HumiditySensorHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetHumiditySensor_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe, callback for humidity change events.
    // The context is a double-boxed pointer the the safe Rust callback.
    unsafe extern "C" fn on_humidity_change(
        chan: HumiditySensorHandle,
        ctx: *mut c_void,
        humidity: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<HumidityChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, humidity);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &HumiditySensorHandle {
        &self.chan
    }

    /// Read the current humidity value.
    pub fn humidity(&self) -> Result<f64> {
        let mut humidity = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getHumidity(self.chan, &mut humidity)
        })?;
        Ok(humidity)
    }

    /// Gets the minimum value the `HumidityChange` event will report.
    pub fn min_humidity(&self) -> Result<f64> {
        let mut humidity = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getMinHumidity(self.chan, &mut humidity)
        })?;
        Ok(humidity)
    }

    /// Gets the maximum value the `HumidityChange` event will report.
    pub fn max_humidity(&self) -> Result<f64> {
        let mut humidity = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getMaxHumidity(self.chan, &mut humidity)
        })?;
        Ok(humidity)
    }

    /// Gets the current value of the `HumidityChangeTrigger`.
    ///
    /// The channel will not issue a HumidityChange event until the
    /// humidity value has changed by the amount specified by the
    /// HumidityChangeTrigger.
    pub fn humidity_change_trigger(&self) -> Result<f64> {
        let mut humidity = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getHumidityChangeTrigger(self.chan, &mut humidity)
        })?;
        Ok(humidity)
    }

    /// Sets the `HumidityChangeTrigger`.
    ///
    /// The channel will not issue a HumidityChange event until the
    /// humidity value has changed by the amount specified by the
    /// HumidityChangeTrigger.
    /// Gets the maximum value the `HumidityChange` event will report.
    ///
    /// Setting this to 0 will result in the channel firing events every
    /// DataInterval. This is useful for applications that implement their
    /// own data filtering.
    pub fn set_humidity_change_trigger(&self, trigger: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_setHumidityChangeTrigger(self.chan, trigger)
        })?;
        Ok(())
    }

    /// Gets the minimum value of the `HumidityChangeTrigger`.
    pub fn min_humidity_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getMinHumidityChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Gets the maximum value of the `HumidityChangeTrigger`.
    pub fn max_humidity_change_trigger(&self) -> Result<f64> {
        let mut trigger = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_getMaxHumidityChangeTrigger(self.chan, &mut trigger)
        })?;
        Ok(trigger)
    }

    /// Sets a handler to receive humitity change callbacks.
    pub fn set_on_humidity_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&HumiditySensor, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<HumidityChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetHumiditySensor_setOnHumidityChangeHandler(
                self.chan,
                Some(Self::on_humidity_change),
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

impl Phidget for HumiditySensor {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for HumiditySensor {}

impl Default for HumiditySensor {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HumiditySensorHandle> for HumiditySensor {
    fn from(chan: HumiditySensorHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for HumiditySensor {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetHumiditySensor_delete(&mut self.chan);
            crate::drop_cb::<HumidityChangeCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
