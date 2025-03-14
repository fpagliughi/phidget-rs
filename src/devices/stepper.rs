// phidget-rs/src/stepper.rs
//
// Copyright (c) 2023, Frank Pagliughi
// implemented by willmendil 2024.
//
// This file is part of the 'phidget-rs' library.
//
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::{AttachCallback, DetachCallback, Error, GenericPhidget, Phidget, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle, PhidgetStepperHandle as StepperHandle};
use std::{
    mem,
    os::raw::{c_int, c_uint, c_void},
    ptr,
    time::Duration,
};

/// The function type for the safe Rust position change callback.
pub type PositionChangeCallback = dyn Fn(&Stepper, f64) + Send + 'static;
/// The function type for the safe Rust velocity change callback.
pub type VelocityChangeCallback = dyn Fn(&Stepper, f64) + Send + 'static;
/// The function type for the safe Rust stop callback.
pub type StoppedCallback = dyn Fn(&Stepper) + Send + 'static;

/// ControlMode for stepper
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(u32)]
pub enum ControlMode {
    /// Control the motor by setting a target position.
    Step = ffi::PhidgetStepper_ControlMode_CONTROL_MODE_STEP,
    /// Control the motor by selecting a target velocity (sign indicates direction).
    /// The motor will rotate continuously in the chosen direction.
    Run = ffi::PhidgetStepper_ControlMode_CONTROL_MODE_RUN,
}

impl TryFrom<u32> for ControlMode {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use ControlMode::*;
        match val {
            ffi::PhidgetStepper_ControlMode_CONTROL_MODE_STEP => Ok(Step),
            ffi::PhidgetStepper_ControlMode_CONTROL_MODE_RUN => Ok(Run),
            _ => Err(ReturnCode::UnknownVal),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Phidget Stepper Motor
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

    /// Enable failsafe for the channel with the specified failsafe time.
    pub fn enable_failsafe(&self, failsafe_time: Duration) -> Result<()> {
        // TODO: Limit to 32-bit or max?
        let ms = u32::try_from(failsafe_time.as_millis()).map_err(|_| ReturnCode::InvalidArg)?;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_enableFailsafe(self.chan, ms) })?;
        Ok(())
    }

    /// Get minimum failsafe time.
    pub fn min_failsafe_time(&self) -> Result<Duration> {
        let mut val: u32 = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinFailsafeTime(self.chan, &mut val) })?;
        Ok(Duration::from_millis(val.into()))
    }

    /// Get maximum failsafe time
    pub fn max_failsafe_time(&self) -> Result<Duration> {
        let mut val: u32 = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMaxFailsafeTime(self.chan, &mut val) })?;
        Ok(Duration::from_millis(val.into()))
    }

    /// Set reset failsafe
    pub fn reset_failsafe(&self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetStepper_resetFailsafe(self.chan) })?;
        Ok(())
    }

    /// Add position offset
    pub fn add_position_offset(&self, position_offset: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_addPositionOffset(self.chan, position_offset)
        })?;
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
    pub fn acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getAcceleration(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum acceleration
    pub fn min_acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinAcceleration(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum acceleration
    pub fn max_acceleration(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMaxAcceleration(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Set control mode
    pub fn set_control_mode(&self, control_mode: ControlMode) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setControlMode(self.chan, control_mode as c_uint)
        })?;
        Ok(())
    }

    /// Get control mode
    pub fn control_mode(&self) -> Result<ControlMode> {
        let mut cm: ffi::PhidgetStepper_ControlMode = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getControlMode(self.chan, &mut cm) })?;
        ControlMode::try_from(cm)
    }

    /// Set current limit
    pub fn set_current_limit(&self, current_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setCurrentLimit(self.chan, current_limit)
        })?;
        Ok(())
    }

    /// Get current limit
    pub fn current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getCurrentLimit(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum current limit
    pub fn min_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum current limit
    pub fn max_current_limit(&self) -> Result<f64> {
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
    pub fn data_interval(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getDataInterval(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum data interval
    pub fn min_data_interval(&self) -> Result<u32> {
        let mut value = 0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinDataInterval(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum data interval
    pub fn max_data_interval(&self) -> Result<u32> {
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
    pub fn data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum data rate
    pub fn min_data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get maximum data rate
    pub fn max_data_rate(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMaxDataRate(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set engaged
    pub fn set_engaged(&self, engaged: bool) -> Result<()> {
        let value = engaged as c_int;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_setEngaged(self.chan, value) })?;
        Ok(())
    }

    /// Get engaged
    pub fn engaged(&self) -> Result<bool> {
        let mut value: c_int = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getEngaged(self.chan, &mut value) })?;
        Ok(value != 0)
    }

    /// Set holding current limit
    pub fn set_holding_current_limit(&self, holding_current_limit: f64) -> Result<()> {
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_setHoldingCurrentLimit(self.chan, holding_current_limit)
        })?;
        Ok(())
    }

    /// Get holding current limit
    pub fn holding_current_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getHoldingCurrentLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get is moving
    pub fn is_moving(&self) -> Result<bool> {
        let mut value = 0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getIsMoving(self.chan, &mut value) })?;
        Ok(value != 0)
    }

    /// Get position
    pub fn position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getPosition(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum position
    pub fn min_position(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getMinPosition(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get maximum position
    pub fn max_position(&self) -> Result<f64> {
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
    pub fn rescale_factor(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getRescaleFactor(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Set target position
    pub fn set_target_position(&self, stepper: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetStepper_setTargetPosition(self.chan, stepper) })?;
        Ok(())
    }

    // /// [NOT IMPLEMENTED] Set target position async TODO
    // pub async fn set_target_position_async(&self, stepper: f64) -> Result<()> {
    //     _ = stepper;
    //     unimplemented!();
    // }

    /// Get target position
    pub fn target_position(&self) -> Result<f64> {
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
    pub fn velocity_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe { ffi::PhidgetStepper_getVelocityLimit(self.chan, &mut value) })?;
        Ok(value)
    }

    /// Get minimum rescale factor
    pub fn min_velocity_limit(&self) -> Result<f64> {
        let mut value = 0.0;
        ReturnCode::result(unsafe {
            ffi::PhidgetStepper_getMinVelocityLimit(self.chan, &mut value)
        })?;
        Ok(value)
    }

    /// Get maximum rescale factor
    pub fn max_velocity_limit(&self) -> Result<f64> {
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
    /// Get the mutable phidget handle for the device
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.chan as PhidgetHandle
    }

    /// Get the immutable/shared phidget handle for the device.
    fn as_handle(&self) -> PhidgetHandle {
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
