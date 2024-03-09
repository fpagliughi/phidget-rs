// phidget-rs/src/humidity_sensor.rs
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
//! Phidget Humidity sensor
//!

use crate::{AttachCallback, DetachCallback, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{
    self as ffi, PhidgetHandle, PhidgetHumiditySensorHandle as HumiditySensorHandle,
};
use std::{mem, os::raw::c_void, ptr};

/// The function signature for the safe Rust humidity change callback.
pub type HumidityCallback = dyn Fn(&HumiditySensor, f64) + Send + 'static;

/// Phidget humidity sensor
pub struct HumiditySensor {
    // Handle to the sensor for the phidget22 library
    chan: HumiditySensorHandle,
    // Double-boxed HumidityCallback, if registered
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
            let cb: &mut Box<HumidityCallback> = &mut *(ctx as *mut _);
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

    /// Sets a handler to receive humitity change callbacks.
    pub fn set_on_humidity_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&HumiditySensor, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<HumidityCallback>> = Box::new(Box::new(cb));
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

impl Phidget for HumiditySensor {
    fn as_handle(&mut self) -> PhidgetHandle {
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
            crate::drop_cb::<HumidityCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
