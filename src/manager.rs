// phidget-rs/src/manager.rs
//
// Copyright (c) 2025 Guillaume Schmid
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//
//! this is the PhidgetManager struct. It allows discovery of the connected
//! phidgets and provides a way to handle connect/disconnect event.
//!

use crate::{ffi, PhidgetRef, Result, ReturnCode};
use phidget_sys::{PhidgetHandle, PhidgetManagerHandle};
use std::os::raw::c_void;
use std::ptr;

/// The signature for device attach callbacks
pub type ManagerAttachCallback = dyn Fn(&PhidgetRef) + Send + 'static;

/// The signature for device detach callbacks
pub type ManagerDetachCallback = dyn Fn(&PhidgetRef) + Send + 'static;

// Low-level, unsafe callback for device attach events
unsafe extern "C" fn on_attach_device(
    _: PhidgetManagerHandle,
    ctx: *mut c_void,
    phid: PhidgetHandle,
) {
    if !ctx.is_null() {
        let cb: &mut Box<ManagerAttachCallback> = &mut *(ctx as *mut _);
        let ph = PhidgetRef::from(phid);
        cb(&ph);
    }
}

// Low-level, unsafe callback for device detach events
unsafe extern "C" fn on_detach_device(
    _: PhidgetManagerHandle,
    ctx: *mut c_void,
    phid: PhidgetHandle,
) {
    if !ctx.is_null() {
        let cb: &mut Box<ManagerDetachCallback> = &mut *(ctx as *mut _);
        let ph = PhidgetRef::from(phid);
        cb(&ph);
    }
}

/// Phidget temperature sensor
pub struct PhidgetManager {
    // Handle to the sensor for the phidget22 library
    mgr: PhidgetManagerHandle,
    // Double-boxed attach callback, if registered
    attach_cb: Option<*mut c_void>,
    // Double-boxed detach callback, if registered
    detach_cb: Option<*mut c_void>,
}

impl PhidgetManager {
    /// Create a new phidget manager
    pub fn new() -> Self {
        let mut mgr: PhidgetManagerHandle = ptr::null_mut();
        unsafe {
            ffi::PhidgetManager_create(&mut mgr);
        }
        Self::from(mgr)
    }

    /// Open the phidget manager.
    pub fn open(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetManager_open(self.mgr) })
    }

    /// Close the phidget manager.
    pub fn close(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::PhidgetManager_close(self.mgr) })
    }

    /// Sets a handler to receive attach callbacks
    pub fn set_on_attach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&PhidgetRef) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<ManagerAttachCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;

        ReturnCode::result(unsafe {
            ffi::PhidgetManager_setOnAttachHandler(self.mgr, Some(on_attach_device), ctx)
        })?;
        self.attach_cb = Some(ctx);
        Ok(())
    }

    /// Sets a handler to receive detach callbacks
    pub fn set_on_detach_handler<F>(&mut self, cb: F) -> Result<()>
    where
        F: Fn(&PhidgetRef) + Send + 'static,
    {
        // 1st box is fat ptr, 2nd is regular pointer.
        let cb: Box<Box<ManagerDetachCallback>> = Box::new(Box::new(cb));
        let ctx = Box::into_raw(cb) as *mut c_void;

        ReturnCode::result(unsafe {
            ffi::PhidgetManager_setOnDetachHandler(self.mgr, Some(on_detach_device), ctx)
        })?;
        self.detach_cb = Some(ctx);
        Ok(())
    }
}

impl Default for PhidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PhidgetManagerHandle> for PhidgetManager {
    fn from(mgr: PhidgetManagerHandle) -> Self {
        PhidgetManager {
            mgr,
            attach_cb: None,
            detach_cb: None,
        }
    }
}

impl Drop for PhidgetManager {
    fn drop(&mut self) {
        let _ = unsafe { ffi::PhidgetManager_close(self.mgr) };
    }
}
