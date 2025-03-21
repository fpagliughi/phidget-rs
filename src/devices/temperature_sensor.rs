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

use crate::{AttachCallback, DetachCallback, Error, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{
    self as ffi, PhidgetHandle, PhidgetTemperatureSensorHandle as TemperatureSensorHandle,
};
use std::{mem, os::raw::c_void, ptr};

/////////////////////////////////////////////////////////////////////////////

/// Phidget Temperature Sensor RTD Types
///
/// A Resistance Temperature Detector (RTD) is a sensor whose
/// resistance changes as its temperature changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum RtdType {
    Pt100_3850 = ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT100_3850,
    Pt1000_3850 = ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT1000_3850,
    Pt100_3920 = ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT100_3920,
    Pt1000_3920 = ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT1000_3920,
}

impl TryFrom<u32> for RtdType {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use RtdType::*;
        match val {
            ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT100_3850 => Ok(Pt100_3850),
            ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT1000_3850 => Ok(Pt1000_3850),
            ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT100_3920 => Ok(Pt100_3920),
            ffi::PhidgetTemperatureSensor_RTDType_RTD_TYPE_PT1000_3920 => Ok(Pt1000_3920),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget Temperature Sensor RTD Wire Setup
///
/// The type of wire setup for TRD sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum RtdWireSetup {
    TwoWire = ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_2WIRE,
    ThreeWire = ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_3WIRE,
    FourWire = ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_4WIRE,
}

impl TryFrom<u32> for RtdWireSetup {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use RtdWireSetup::*;
        match val {
            ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_2WIRE => Ok(TwoWire),
            ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_3WIRE => Ok(ThreeWire),
            ffi::Phidget_RTDWireSetup_RTD_WIRE_SETUP_4WIRE => Ok(FourWire),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget Temperature Sensor Thermocouple Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum ThermocoupleType {
    TypeJ = ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_J,
    TypeK = ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_K,
    TypeE = ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_E,
    TypeT = ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_T,
}

impl TryFrom<u32> for ThermocoupleType {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use ThermocoupleType::*;
        match val {
            ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_J => Ok(TypeJ),
            ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_K => Ok(TypeK),
            ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_E => Ok(TypeE),
            ffi::PhidgetTemperatureSensor_ThermocoupleType_THERMOCOUPLE_TYPE_T => Ok(TypeT),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// The function type for the safe Rust temperature change callback.
pub type TemperatureCallback = dyn Fn(&TemperatureSensor, f64) + Send + 'static;

/// Phidget temperature sensor
pub struct TemperatureSensor {
    // Handle to the sensor for the phidget22 library
    chan: TemperatureSensorHandle,
    // Double-boxed TemperatureCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl TemperatureSensor {
    /// Create a new temperature sensor.
    pub fn new() -> Self {
        let mut chan: TemperatureSensorHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetTemperatureSensor_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe, callback for temperature change events.
    // The context is a double-boxed pointer the the safe Rust callback.
    unsafe extern "C" fn on_temperature_change(
        chan: TemperatureSensorHandle,
        ctx: *mut c_void,
        temperature: f64,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<TemperatureCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, temperature);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &TemperatureSensorHandle {
        &self.chan
    }

    /// Get the RTD sensor type
    pub fn rtd_type(&self) -> Result<RtdType> {
        let mut typ: u32 = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_getRTDType(self.chan, &mut typ)
        })?;
        RtdType::try_from(typ)
    }

    /// Set the RTD sensor type
    pub fn set_rtd_type(&mut self, typ: RtdType) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_setRTDType(self.chan, typ as u32)
        })?;
        Ok(())
    }

    /// Get the RTD wire setup
    pub fn rtd_wire_setup(&self) -> Result<RtdWireSetup> {
        let mut typ: u32 = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_getRTDWireSetup(self.chan, &mut typ)
        })?;
        RtdWireSetup::try_from(typ)
    }

    /// Set the RTD wire setup
    pub fn set_rtd_wire_setup(&mut self, typ: RtdWireSetup) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_setRTDWireSetup(self.chan, typ as u32)
        })?;
        Ok(())
    }

    /// Get the type of thermocouple
    pub fn thermocouple_type(&self) -> Result<ThermocoupleType> {
        let mut typ: u32 = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_getThermocoupleType(self.chan, &mut typ)
        })?;
        ThermocoupleType::try_from(typ)
    }

    /// Set the type of thermocouple
    pub fn set_thermocouple_type(&mut self, typ: ThermocoupleType) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_setThermocoupleType(self.chan, typ as u32)
        })?;
        Ok(())
    }

    /// Read the current temperature
    pub fn temperature(&self) -> Result<f64> {
        let mut temperature = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_getTemperature(self.chan, &mut temperature)
        })?;
        Ok(temperature)
    }

    /// Set a handler to receive temperature change callbacks.
    pub fn set_on_temperature_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&TemperatureSensor, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<TemperatureCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetTemperatureSensor_setOnTemperatureChangeHandler(
                self.chan,
                Some(Self::on_temperature_change),
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

impl Phidget for TemperatureSensor {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for TemperatureSensor {}

impl Default for TemperatureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TemperatureSensorHandle> for TemperatureSensor {
    fn from(chan: TemperatureSensorHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for TemperatureSensor {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetTemperatureSensor_delete(&mut self.chan);
            crate::drop_cb::<TemperatureCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
