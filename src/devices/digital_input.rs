// phidget-rs/src/digital_input.rs
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

use crate::{AttachCallback, DetachCallback, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetDigitalInputHandle, PhidgetHandle};
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
    chan: PhidgetDigitalInputHandle,
    // Double-boxed DigitalInputCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

/// InputMode for digital input
/// <http://perk-software.cs.queensu.ca/plus/doc/nightly/dev/phidget22_8h.html#a5ad0740978daad6539d3a8249607bd46>
#[derive(Copy, Clone)]
pub enum InputMode {
    /// For using sensors with NPN transistor outputs.
    NPN,
    /// For using sensors with PNP transistor outputs.
    PNP,
}

/// PowerSupply for digital input
/// <http://perk-software.cs.queensu.ca/plus/doc/nightly/dev/phidget22_8h.html#a0293d3a21e8de247c4b562ceda897876>
#[derive(Copy, Clone)]
pub enum PowerSupply {
    /// OFF: cannot find docs
    OFF,
    /// V12: cannot find docs
    V12,
    /// v24: cannot find docs
    V24,
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

    /// Set input mode
    pub fn set_input_mode(&self, input_mode: InputMode) -> Result<()> {
        let v = match input_mode {
            InputMode::NPN => 1,
            InputMode::PNP => 0,
        };

        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_setInputMode(self.chan, v) })?;
        Ok(())
    }

    /// Get input mode
    pub fn input_mode(&self) -> Result<InputMode> {
        let mut im: ffi::Phidget_InputMode = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getInputMode(self.chan, &mut im) })?;

        match im {
            1 => Ok(InputMode::NPN),
            0 => Ok(InputMode::PNP),
            _ => Err(ReturnCode::UnknownVal),
        }
    }

    /// Set power supply
    pub fn set_power_supply(&self, power_supply: PowerSupply) -> Result<()> {
        let v: u32 = match power_supply {
            PowerSupply::OFF => 1,
            PowerSupply::V12 => 2,
            PowerSupply::V24 => 3,
        };

        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_setPowerSupply(self.chan, v) })?;
        Ok(())
    }

    /// Get power supply
    pub fn power_supply(&self) -> Result<PowerSupply> {
        let mut ps: ffi::Phidget_PowerSupply = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getPowerSupply(self.chan, &mut ps) })?;

        match ps {
            1 => Ok(PowerSupply::OFF),
            2 => Ok(PowerSupply::V12),
            3 => Ok(PowerSupply::V24),
            _ => Err(ReturnCode::UnknownVal),
        }
    }

    /// Get the state of the digital input channel
    pub fn state(&self) -> Result<bool> {
        let mut v = 0;
        ReturnCode::result(unsafe { ffi::PhidgetDigitalInput_getState(self.chan, &mut v) })?;

        match v {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(ReturnCode::UnknownVal),
        }
    }

    // ---------------------------------------------------

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
            cb(&sensor, state as i32);
            mem::forget(sensor);
        }
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &PhidgetDigitalInputHandle {
        &self.chan
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

impl Phidget for DigitalInput {
    fn as_handle(&mut self) -> PhidgetHandle {
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
