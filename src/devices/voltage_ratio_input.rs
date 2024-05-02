use crate::{AttachCallback, DetachCallback, Error, GenericPhidget, Phidget, Result, ReturnCode, VoltageInput};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetVoltageInputHandle, PhidgetVoltageRatioInputHandle};
use std::{mem, os::raw::c_void, ptr};
use log::error;

pub type VoltageRatioChangeCallback = dyn Fn(&VoltageRatioInput, f64) + Send + 'static;

/// The function signature for the safe Rust voltage ratio change callback.
pub struct VoltageRatioInput {
    // Handle to the voltage ratio input in the phidget22 libary
    chan: PhidgetVoltageRatioInputHandle,
    // Double-boxed VoltageRatioChangeCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>
}

impl VoltageRatioInput {
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
    pub fn as_channel(&self) -> &PhidgetVoltageRatioInputHandle {& self.chan}

    /// Get the voltage on the input channel
    pub fn set_data_interval(&self, data_interval: u32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetVoltageRatioInput_setDataInterval(self.chan, data_interval) })?;
        Ok(())
    }

    pub fn get_min_data_interval(&self) -> Result<u32> {
        let mut min_data_interval: u32 = 0;
        ReturnCode::result(unsafe { ffi::PhidgetVoltageRatioInput_getMinDataInterval(self.chan, &mut min_data_interval) })?;
        Ok(min_data_interval)
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


