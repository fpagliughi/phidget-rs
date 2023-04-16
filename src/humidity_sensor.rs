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

use crate::{Phidget, Result};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetHumiditySensorHandle as HumiditySensorHandle};
use std::{mem, os::raw::c_void, ptr};


pub type OnHumidityChangeCallback = dyn Fn(&HumiditySensor, f64) + Send + 'static;

pub struct HumiditySensor {
    chan: HumiditySensorHandle,
    cb: Option<Box<OnHumidityChangeCallback>>,
}

impl HumiditySensor {
    /// Create a new humidity sensor.
    pub fn new() -> Self {
        let mut chan: HumiditySensorHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetHumiditySensor_create(&mut chan);
        }
        Self { chan, cb: None }
    }

    // Low-level, unsafe, callback for humidity change events.
    // The context is a pointer the the Rust `HumiditySensor` object.
    unsafe extern "C" fn on_humidity_change(
        chan: HumiditySensorHandle,
        ctx: *mut c_void,
        humidity: f64,
    ) {
        if ctx.is_null() {
            return;
        }
        let psensor = ctx as *mut HumiditySensor;
        let sensor = Self { chan, cb: None };

        if let Some(ref cb) = (*psensor).cb {
            cb(&sensor, humidity);
        }
        mem::forget(sensor);
    }

    pub fn as_channel(&self) -> &HumiditySensorHandle {
        &self.chan
    }

    /// Read the current humidity value.
    pub fn humidity(&self) -> Result<f64> {
        let mut humidity = 0.0;
        unsafe {
            crate::check_ret(ffi::PhidgetHumiditySensor_getHumidity(self.chan, &mut humidity))?;
        }
        Ok(humidity)
    }

    /// Sets a handler to receive humitity change callbacks.
    pub fn set_on_humidity_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&HumiditySensor, f64) + Send + 'static,
    {
        self.cb = Some(Box::new(cb));
        let this = self as *mut _ as *mut c_void;
        unsafe {
            crate::check_ret(ffi::PhidgetHumiditySensor_setOnHumidityChangeHandler(
                self.chan,
                Some(Self::on_humidity_change),
                this
            ))
        }
    }

    /// Removes the change callback.
    pub fn remove_on_humidity_change_handler(&mut self) -> Result<()> {
        self.cb = None;
        unsafe {
            crate::check_ret(ffi::PhidgetHumiditySensor_setOnHumidityChangeHandler(
                self.chan,
                None,
                ptr::null_mut()
            ))
        }
    }
}

impl Phidget for HumiditySensor {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Drop for HumiditySensor {
    fn drop(&mut self) {
        let _ = self.remove_on_humidity_change_handler();
        unsafe {
            ffi::PhidgetHumiditySensor_delete(&mut self.chan);
        }
    }
}

