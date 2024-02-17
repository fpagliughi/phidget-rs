// phidget-rs/src/stepper.rs
//
// Copyright (c) 2023, Frank Pagliughi
// implemented by Guanicoe 2024.
//
// This file is part of the 'phidget-rs' library.
//
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::{AttachCallback, DetachCallback, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetStepperHandle as StepperHandle};
use std::{mem, os::raw::c_void, ptr};

/// The function type for the safe Rust position change callback.
pub type PositionChangeCallback = dyn Fn(&Stepper, f64) + Send + 'static;
/// The function type for the safe Rust velocity change callback.
pub type VelocityChangeCallback = dyn Fn(&Stepper, f64) + Send + 'static;
/// The function type for the safe Rust stop callback.
pub type StoppedCallback = dyn Fn(&Stepper) + Send + 'static;

/// Phidget Stepper sensor
pub struct Stepper {
    // Handle to the sensor for the phidget22 library
    chan: StepperHandle,
    // Double-boxed StepperCallback, if registered
    cb: Option<*mut c_void>,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

/// ControlMode for stepper
#[derive(Copy, Clone)]
pub enum ControlMode {
    /// Step: Control the motor by setting a target position.
    Step,
    /// Run: Control the motor by selecting a target velocity (sign indicates direction). The motor will rotate continuously in the chosen direction.
    Run,
}

impl Stepper {
    /// Create a new Stepper sensor.
    pub fn new() -> Self {
        let mut chan: StepperHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetStepper_create(&mut chan);
        }
        Self::from(chan)
    }

    /// Get a reference to the underlying sensor handle
    pub fn as_channel(&self) -> &StepperHandle {
        &self.chan
    }

    /// Set enable failsafe
    pub fn set_enable_failsafe(&self, failsafe_time: u32) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_enableFailsafe(self.chan, failsafe_time)
        })?;
        Ok(())
    }

    /// Add position offset
    pub fn get_add_position_offset(&self, position_offset: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_addPositionOffset(self.chan, position_offset)
        })?;
        Ok(())
    }

    /// Set reset failsafe
    pub fn set_reset_failsafe(&self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetStepper_resetFailsafe(self.chan) })?;
        Ok(())
    }
    /// Set acceleration
    pub fn set_acceleration(&self, acceleration: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setAcceleration(self.chan, acceleration)
        })?;
        Ok(())
    }

    /// Get acceleration
    pub fn get_acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getAcceleration(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum acceleration
    pub fn get_min_acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinAcceleration(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum acceleration
    pub fn get_max_acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMaxAcceleration(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set control mode
    pub fn set_control_mode(&self, control_mode: ControlMode) -> Result<()> {
        let _cm = match control_mode {
            ControlMode::Step => 0,
            ControlMode::Run => 1,
        };

        ReturnCode::result(unsafe { ffi::PhidgetStepper_setControlMode(self.chan, _cm) })?;
        Ok(())
    }

    /// Get control mode
    pub fn get_control_mode(&self) -> Result<ControlMode> {
        let mut _cm: ffi::PhidgetStepper_ControlMode = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getControlMode(self.chan, &mut _cm) })?;

        match _cm {
            1 => Ok(ControlMode::Run),
            0 => Ok(ControlMode::Step),
            _ => Err(ReturnCode::UnknownVal),
        }
    }

    /// Set current limit
    pub fn set_current_limit(&self, current_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setCurrentLimit(self.chan, current_limit)
        })?;
        Ok(())
    }
    /// Get current limit
    pub fn get_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getCurrentLimit(self.chan, &mut value) })?;
        Ok(value)
    }
    /// Get minimum current limit
    pub fn get_min_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }
    /// Get maximum current limit
    pub fn get_max_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMaxCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set data interval
    pub fn set_data_interval(&self, data_interval: u32) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setDataInterval(self.chan, data_interval)
        })?;
        Ok(())
    }
    /// Get data interval
    pub fn get_data_interval(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getDataInterval(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum data interval
    pub fn get_min_data_interval(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinDataInterval(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum data interval
    pub fn get_max_data_interval(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMaxDataInterval(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set data rate
    pub fn set_data_rate(&self, data_rate: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetStepper_setDataRate(self.chan, data_rate) })?;
        Ok(())
    }
    /// Get data rate
    pub fn get_data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum data rate
    pub fn get_min_data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get maximum data rate
    pub fn get_max_data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMaxDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set engaged
    pub fn set_engaged(&self, engaged: bool) -> Result<()> {
        let _v = match engaged {
            true => 1,
            false => 0,
        };

        ReturnCode::result(unsafe { ffi::PhidgetStepper_setEngaged(self.chan, _v) })?;
        Ok(())
    }

    /// Get engaged
    pub fn get_engaged(&self) -> Result<bool> {
        let mut _v = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getEngaged(self.chan, &mut _v) })?;

        match _v {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(ReturnCode::UnknownVal),
        }
    }
    /// Get minimum data rate
    pub fn get_min_failsafe_time(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get maximum data rate
    pub fn get_max_failsafe_time(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMaxDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set holding current limit
    pub fn set_holding_current_limit(&self, holding_current_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setHoldingCurrentLimit(self.chan, holding_current_limit)
        })?;
        Ok(())
    }

    /// Get holding current limit
    pub fn get_holding_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getHoldingCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get is moving
    pub fn get_is_moving(&self) -> Result<bool> {
        let mut _v = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getIsMoving(self.chan, &mut _v) })?;

        match _v {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(ReturnCode::UnknownVal),
        }
    }

    /// Get position
    pub fn get_position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getPosition(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum position
    pub fn get_min_position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinPosition(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get maximum position
    pub fn get_max_position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMaxPosition(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set rescale factor
    pub fn set_rescale_factor(&self, rescale_factor: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setRescaleFactor(self.chan, rescale_factor)
        })?;
        Ok(())
    }

    /// Get rescale factor
    pub fn get_rescale_factor(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getRescaleFactor(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set target position
    pub fn set_target_position(&self, stepper: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetStepper_setTargetPosition(self.chan, stepper) })?;
        Ok(())
    }
    /// Set target position async TODO
    pub async fn set_target_position_async(&self, stepper: f64) -> Result<()> {
        _ = stepper;
        unimplemented!();
    }
    /// Get target position
    pub fn get_target_position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getTargetPosition(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set velocity limit
    pub fn set_velocity_limit(&self, velocity_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setVelocityLimit(self.chan, velocity_limit)
        })?;
        Ok(())
    }

    /// Get rescale factor
    pub fn get_velocity_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getVelocityLimit(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum rescale factor
    pub fn get_min_velocity_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinVelocityLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum rescale factor
    pub fn get_max_velocity_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMaxVelocityLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    // Low-level, unsafe, callback for position change events.
    // The context is a double-boxed pointer the safe Rust callback.
    unsafe extern "C" fn on_position_change(chan: StepperHandle, ctx: *mut c_void, stepper: f64) {
        if !ctx.is_null() {
            let cb: &mut Box<PositionChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, stepper);
            mem::forget(sensor);
        }
    }

    /// Set a handler to receive position change callbacks.
    pub fn set_on_position_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&Stepper, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<PositionChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setOnPositionChangeHandler(
                self.chan,
                Some(Self::on_position_change),
                ctx,
            )
        })
    }

    // Low-level, unsafe, callback for stop events.
    // The context is a double-boxed pointer the safe Rust callback.
    unsafe extern "C" fn on_stopped(chan: StepperHandle, ctx: *mut c_void) {
        if !ctx.is_null() {
            let cb: &mut Box<StoppedCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor);
            mem::forget(sensor);
        }
    }

    /// Set a handler to receive stop callbacks.
    pub fn set_on_stopped_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&Stepper) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<StoppedCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setOnStoppedHandler(self.chan, Some(Self::on_stopped), ctx)
        })
    }

    // Low-level, unsafe, callback for velocity change events.
    // The context is a double-boxed pointer the safe Rust callback.
    unsafe extern "C" fn on_velocity_change(chan: StepperHandle, ctx: *mut c_void, stepper: f64) {
        if !ctx.is_null() {
            let cb: &mut Box<VelocityChangeCallback> = &mut *(ctx as *mut _);
            let sensor = Self::from(chan);
            cb(&sensor, stepper);
            mem::forget(sensor);
        }
    }

    /// Set a handler to receive stepper change callbacks.
    pub fn set_on_velocity_change_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&Stepper, f64) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<VelocityChangeCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;
        self.cb = Some(ctx);

        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setOnVelocityChangeHandler(
                self.chan,
                Some(Self::on_velocity_change),
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

impl Phidget for Stepper {
    fn as_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }
}

unsafe impl Send for Stepper {}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StepperHandle> for Stepper {
    fn from(chan: StepperHandle) -> Self {
        Self {
            chan,
            cb: None,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for Stepper {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe {
            ffi::PhidgetStepper_delete(&mut self.chan);
            crate::drop_cb::<PositionChangeCallback>(self.cb.take());
            crate::drop_cb::<VelocityChangeCallback>(self.cb.take());
            crate::drop_cb::<StoppedCallback>(self.cb.take());
            crate::drop_cb::<AttachCallback>(self.attach_cb.take());
            crate::drop_cb::<DetachCallback>(self.detach_cb.take());
        }
    }
}
