// phidget-rs/src/digital_input.rs
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

use crate::{Error, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetDigitalInputHandle, PhidgetHandle};
use std::{
    ffi::{c_int, c_uint, c_void},
    mem, ptr,
};

/// InputMode for digital input
/// <http://perk-software.cs.queensu.ca/plus/doc/nightly/dev/phidget22_8h.html#a5ad0740978daad6539d3a8249607bd46>
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum InputMode {
    /// For using sensors with NPN transistor outputs.
    NPN = ffi::Phidget_InputMode_INPUT_MODE_NPN,
    /// For using sensors with PNP transistor outputs.
    PNP = ffi::Phidget_InputMode_INPUT_MODE_PNP,
}

impl TryFrom<u32> for InputMode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        use InputMode::*;
        match value {
            ffi::Phidget_InputMode_INPUT_MODE_NPN => Ok(NPN),
            ffi::Phidget_InputMode_INPUT_MODE_PNP => Ok(PNP),
            _ => Err(ReturnCode::UnknownVal),
        }
    }
}

/// PowerSupply for digital input
/// <http://perk-software.cs.queensu.ca/plus/doc/nightly/dev/phidget22_8h.html#a0293d3a21e8de247c4b562ceda897876>
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum PowerSupply {
    /// Power supply off
    OFF = ffi::Phidget_PowerSupply_POWER_SUPPLY_OFF,
    /// 12V power supply
    V12 = ffi::Phidget_PowerSupply_POWER_SUPPLY_12V,
    /// 24V power supply
    V24 = ffi::Phidget_PowerSupply_POWER_SUPPLY_24V,
}

impl TryFrom<u32> for PowerSupply {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        use PowerSupply::*;
        match value {
            ffi::Phidget_PowerSupply_POWER_SUPPLY_OFF => Ok(OFF),
            ffi::Phidget_PowerSupply_POWER_SUPPLY_12V => Ok(V12),
            ffi::Phidget_PowerSupply_POWER_SUPPLY_24V => Ok(V24),
            _ => Err(ReturnCode::UnknownVal),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// The function type for the safe Rust temperature sensor attach callback.
pub type AttachCallback = dyn Fn(&mut DigitalInput) + Send + 'static;

/// The function type for the safe Rust temperature sensor detach callback.
pub type DetachCallback = dyn Fn(&mut DigitalInput) + Send + 'static;

/// The function signature for the safe Rust digital input state change callback.
pub type DigitalInputCallback = dyn Fn(&DigitalInput, u8) + Send + 'static;

/// Phidget digital input
pub struct DigitalInput {
    // Handle to the digital input in the phidget22 library
    chan: PhidgetDigitalInputHandle,
    // Double-boxed DigitalInputCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl DigitalInput {
    /// Create a new digital input.
    pub fn new() -> Self {
        let mut chan: PhidgetDigitalInputHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetDigitalInput_create(&mut chan);
        }
        Self::from(chan)
    }

    // Low-level, unsafe callback for device attach events
    unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetDigitalInputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe callback for device detach events
    unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
            let mut sensor = Self::from(phid as PhidgetDigitalInputHandle);
            cb(&mut sensor);
            mem::forget(sensor);
        }
    }

    // Low-level, unsafe, callback for the digital input state change event.
    // The context is a double-boxed pointer to the safe Rust callback.
    unsafe extern "C" fn on_state_change(
        chan: PhidgetDigitalInputHandle,
        ctx: *mut c_void,
        state: c_int,
    ) {
        if !ctx.is_null() {
            let cb: &mut Box<DigitalInputCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, state as u8);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetDigitalInputHandle {
        &self.chan
    }

    /// Set input mode
    pub fn set_input_mode(&self, input_mode: InputMode) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalInput_setInputMode(self.chan, input_mode as c_uint)
        })?;
        Ok(())
    }

    /// Get input mode
    pub fn input_mode(&self) -> Result<InputMode> {
        let mut im: ffi::Phidget_InputMode = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getInputMode(self.chan, &mut im) })?;
        InputMode::try_from(im)
    }

    /// Set the type of power supply.
    ///
    /// - Set this to the voltage specified in the attached sensor's data sheet to power it.
    /// - Set to `PowerSupply::Off` to turn off the supply to save power.
    pub fn set_power_supply(&self, power_supply: PowerSupply) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetDigitalInput_setPowerSupply(self.chan, power_supply as c_uint)
        })?;
        Ok(())
    }

    /// Get type of power supply
    pub fn power_supply(&self) -> Result<PowerSupply> {
        let mut ps: ffi::Phidget_PowerSupply = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getPowerSupply(self.chan, &mut ps) })?;
        PowerSupply::try_from(ps)
    }

    /// Get the state of the digital input channel
    pub fn state(&self) -> Result<u8> {
        let mut value = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getState(self.chan, &mut value) })?;
        Ok(value as u8)
    }

    /// Sets a handler to receive digital input state change callbacks.
    pub fn set_on_state_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&DigitalInput, u8) + Send + 'static,
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

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&mut DigitalInput) + Send + 'static,
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
        F: Fn(&mut DigitalInput) + Send + 'static,
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

impl Phidget for DigitalInput {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for DigitalInput {}

impl Default for DigitalInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetDigitalInputHandle> for DigitalInput {
    fn from(chan: PhidgetDigitalInputHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetDigitalInput_delete(&mut self.chan);
            crate::drop_cb::<DigitalInputCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
