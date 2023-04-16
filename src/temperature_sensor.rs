// phidget-rs/src/temperature_sensor.rs
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
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetTemperatureSensorHandle as TemperatureSensorHandle};
use std::{os::raw::c_void, ptr};


pub type OnTemperatureChangeCallback = dyn Fn(&TemperatureSensor, f64) + Send + 'static;

pub struct TemperatureSensor {
    chan: TemperatureSensorHandle,
    cb: Option<Box<OnTemperatureChangeCallback>>,
}

impl TemperatureSensor {
    /// Create a new temperature sensor.
    pub fn new() -> Self {
        let mut chan: TemperatureSensorHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetTemperatureSensor_create(&mut chan);
        }
        Self { chan, cb: None }
    }

    unsafe extern "C" fn on_temperature_change(
        _chan: TemperatureSensorHandle,
        ctx: *mut c_void,
        temperature: f64,
    ) {
        if ctx.is_null() {
            return;
        }
        let sensor = ctx as *mut TemperatureSensor;
    }

    pub fn as_channel(&self) -> &TemperatureSensorHandle {
        &self.chan
    }

    pub fn temperature(&self) -> Result<f64> {
        let mut temperature = 0.0;
        unsafe {
            crate::check_ret(ffi::PhidgetTemperatureSensor_getTemperature(self.chan, &mut temperature))?;
        }
        Ok(temperature)
    }

    pub fn set_on_temperature_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&TemperatureSensor, f64) + Send + 'static,
    {
        self.cb = Some(Box::new(cb));
        let this = self as *mut _ as *mut c_void;
        unsafe {
            crate::check_ret(ffi::PhidgetTemperatureSensor_setOnTemperatureChangeHandler(
                self.chan,
                Some(Self::on_temperature_change),
                this
            ))
        }
    }

    pub fn remove_on_temperature_change_handler(&mut self) -> Result<()> {
        self.cb = None;
        unsafe {
            crate::check_ret(ffi::PhidgetTemperatureSensor_setOnTemperatureChangeHandler(
                self.chan,
                None,
                ptr::null_mut()
            ))
        }
    }
}

impl Phidget for TemperatureSensor {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

impl Drop for TemperatureSensor {
    fn drop(&mut self) {
        let _ = self.remove_on_temperature_change_handler();
        unsafe {
            ffi::PhidgetTemperatureSensor_delete(&mut self.chan);
        }
    }
}

