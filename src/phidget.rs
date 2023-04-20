// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::{ChannelClass, DeviceClass, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle};
use std::{
    os::raw::{c_int, c_void},
    time::Duration,
};

/// The signature for device attach callbacks
pub type AttachCallback = dyn Fn(&GenericPhidget) + Send + 'static;

/// The signature for device detach callbacks
pub type DetachCallback = dyn Fn(&GenericPhidget) + Send + 'static;

// Low-level, unsafe callback for device attach events
unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
    if !ctx.is_null() {
        let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
        let ph = GenericPhidget::from(phid);
        cb(&ph);
    }
}

// Low-level, unsafe callback for device detach events
unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
    if !ctx.is_null() {
        let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
        let ph = GenericPhidget::from(phid);
        cb(&ph);
    }
}

// ----- Callbacks -----

/// Assigns a handler that will be called when the Attach event occurs for
/// a matching phidget.
pub fn set_on_attach_handler<P, F>(ph: &mut P, cb: F) -> Result<*mut c_void>
where
    P: Phidget,
    F: Fn(&GenericPhidget) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<AttachCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::Phidget_setOnAttachHandler(ph.as_handle(), Some(on_attach), ctx)
    })?;
    Ok(ctx)
}

/// Assigns a handler that will be called when the Detach event occurs for
/// a matching Phidget.
pub fn set_on_detach_handler<P, F>(ph: &mut P, cb: F) -> Result<*mut c_void>
where
    P: Phidget,
    F: Fn(&GenericPhidget) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<DetachCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::Phidget_setOnDetachHandler(ph.as_handle(), Some(on_detach), ctx)
    })?;
    Ok(ctx)
}

/////////////////////////////////////////////////////////////////////////////

/// The base trait and implementation for Phidgets
pub trait Phidget: Send {
    /// Get the phidget handle for the device
    fn as_handle(&mut self) -> PhidgetHandle;

    /// Attempt to open the humidity sensor for input.
    fn open(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_open(self.as_handle()) })
    }

    /// Attempt to open the humidity sensor for input, waiting a limited time
    /// for it to connect.
    fn open_wait(&mut self, to: Duration) -> Result<()> {
        let ms = to.as_millis() as u32;
        ReturnCode::result(unsafe { ffi::Phidget_openWaitForAttachment(self.as_handle(), ms) })
    }

    /// Attempt to open the humidity sensor for input, waiting the default time
    /// for it to connect.
    fn open_wait_default(&mut self) -> Result<()> {
        self.open_wait(crate::TIMEOUT_DEFAULT)
    }

    /// Closes the channel
    fn close(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_close(self.as_handle()) })
    }

    /// Determines if the channel is open
    fn is_open(&mut self) -> Result<bool> {
        let mut open: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsOpen(self.as_handle(), &mut open) })?;
        Ok(open != 0)
    }

    /// Determines if the channel is open and attached to a device.
    fn is_attached(&mut self) -> Result<bool> {
        let mut attached: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getAttached(self.as_handle(), &mut attached) })?;
        Ok(attached != 0)
    }

    /// Determines if the channel is open locally (not over a network).
    fn is_local(&mut self) -> Result<bool> {
        let mut local: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsLocal(self.as_handle(), &mut local) })?;
        Ok(local != 0)
    }

    /// Set true to open the channel locally (not over a network).
    fn set_local(&mut self, local: bool) -> Result<()> {
        let local = c_int::from(local);
        ReturnCode::result(unsafe { ffi::Phidget_setIsLocal(self.as_handle(), local) })
    }

    /// Determines if the channel is open remotely (over a network).
    fn is_remote(&mut self) -> Result<bool> {
        let mut rem: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsRemote(self.as_handle(), &mut rem) })?;
        Ok(rem != 0)
    }

    /// Set true to open the channel locally,  (not over a network).
    fn set_remote(&mut self, rem: bool) -> Result<()> {
        let rem = c_int::from(rem);
        ReturnCode::result(unsafe { ffi::Phidget_setIsRemote(self.as_handle(), rem) })
    }

    /// Gets the data interval for the device, if supported.
    fn data_interval(&mut self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms as u64))
    }

    /// Sets the data interval for the device, if supported.
    fn set_data_interval(&mut self, interval: Duration) -> Result<()> {
        let ms = interval.as_millis() as u32;
        ReturnCode::result(unsafe { ffi::Phidget_setDataInterval(self.as_handle(), ms) })
    }

    /// Gets the minimum data interval for the device, if supported.
    fn min_data_interval(&mut self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getMinDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms as u64))
    }

    /// Gets the maximum data interval for the device, if supported.
    fn max_data_interval(&mut self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getMaxDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms as u64))
    }

    /// Gets the data update rate for the device, if supported.
    fn data_rate(&mut self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Sets the data update rate for the device, if supported.
    fn set_data_rate(&mut self, freq: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setDataRate(self.as_handle(), freq) })
    }

    /// Gets the minimum data interval for the device, if supported.
    fn min_data_rate(&mut self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getMinDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Gets the maximum data interval for the device, if supported.
    fn max_data_rate(&mut self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getMaxDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Get the number of channels of the specified class on the device.
    fn device_channel_count(&mut self, cls: ChannelClass) -> Result<u32> {
        let mut n: u32 = 0;
        let cls = cls as ffi::Phidget_ChannelClass;
        ReturnCode::result(unsafe {
            ffi::Phidget_getDeviceChannelCount(self.as_handle(), cls, &mut n)
        })?;
        Ok(n)
    }

    /// Gets class of the channel
    fn channel_class(&mut self) -> Result<ChannelClass> {
        let mut cls = ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING;
        ReturnCode::result(unsafe { ffi::Phidget_getChannelClass(self.as_handle(), &mut cls) })?;
        ChannelClass::try_from(cls)
    }

    /// Get the name of the channel class
    fn channel_class_name(&mut self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getChannelClassName(self.as_handle(), s) })
    }

    /// Get the channel's name.
    fn channel_name(&mut self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getChannelName(self.as_handle(), s) })
    }

    /// Gets class of the device
    fn device_class(&mut self) -> Result<DeviceClass> {
        let mut cls = ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING;
        ReturnCode::result(unsafe { ffi::Phidget_getDeviceClass(self.as_handle(), &mut cls) })?;
        DeviceClass::try_from(cls)
    }

    /// Get the name of the device class
    fn device_class_name(&mut self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getDeviceClassName(self.as_handle(), s) })
    }

    // ----- Filters -----

    /// Determines whether this channel is a VINT Hub port channel, or part
    /// of a VINT device attached to a hub port.
    fn is_hub_port_device(&mut self) -> Result<bool> {
        let mut on: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsHubPortDevice(self.as_handle(), &mut on) })?;
        Ok(on != 0)
    }

    /// Specify whether this channel should be opened on a VINT Hub port
    /// directly, or on a VINT device attached to a hub port.
    /// This must be set before the channel is opened.
    fn set_is_hub_port_device(&mut self, on: bool) -> Result<()> {
        let on = c_int::from(on);
        ReturnCode::result(unsafe { ffi::Phidget_setIsHubPortDevice(self.as_handle(), on) })
    }

    /// Gets the index of the port on the VINT Hub to which the channel is attached.
    fn hub_port(&mut self) -> Result<i32> {
        let mut port: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getHubPort(self.as_handle(), &mut port) })?;
        Ok(port as i32)
    }

    /// Gets the index of the port on the VINT Hub to which the channel is attached.
    /// Set to PHIDGET_HUBPORT_ANY to open the channel on any port of the hub.
    /// This must be set before the channel is opened.
    fn set_hub_port(&mut self, port: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setHubPort(self.as_handle(), port as c_int) })
    }

    /// Gets the channel index of the device.
    fn channel(&mut self) -> Result<i32> {
        let mut ch: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getChannel(self.as_handle(), &mut ch) })?;
        Ok(ch as i32)
    }

    /// Sets the channel index to be opened.
    /// The default channel is 0. Set to PHIDGET_CHANNEL_ANY to open any
    /// channel on the specified device. This must be set before the channel
    /// is opened.
    fn set_channel(&mut self, chan: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setChannel(self.as_handle(), chan as c_int) })
    }

    /// Gets the serial number of the device.
    /// If the channel is part of a VINT device, this is the serial number
    /// of the VINT Hub to which the device is attached.
    fn serial_number(&mut self) -> Result<i32> {
        let mut n = 0;
        ReturnCode::result(unsafe {
            ffi::Phidget_getDeviceSerialNumber(self.as_handle(), &mut n)
        })?;
        Ok(n)
    }

    /// Sets the device serial number to be opened.
    /// Leave un-set, or set to PHIDGET_SERIALNUMBER_ANY to open any serial
    /// number. If the channel is part of a VINT device, this is the serial
    /// number of the VINT Hub to which the device is attached.
    /// This must be set before the channel is opened.
    fn set_serial_number(&mut self, sn: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setDeviceSerialNumber(self.as_handle(), sn) })
    }
}

/////////////////////////////////////////////////////////////////////////////

/// A wrapper for a generic phidget.
///
/// This contains a wrapper around a generic PhidgetHandle, which might be
/// any type of device. It can be queried for additional information and
/// potentially converted into a specific device object.
///
/// This is a non-owning object. It will not release the underlying Phidget
/// when dropped. It is typically used to wrap a generic handle sent to a
/// callback from the phidget22 library.
pub struct GenericPhidget {
    phid: PhidgetHandle,
}

impl GenericPhidget {
    /// Creates a new, generic phidget for the handle.
    pub fn new(phid: PhidgetHandle) -> Self {
        Self { phid }
    }
}

impl Phidget for GenericPhidget {
    /// Get the phidget handle for the device
    fn as_handle(&mut self) -> PhidgetHandle {
        self.phid
    }
}

unsafe impl Send for GenericPhidget {}

impl From<PhidgetHandle> for GenericPhidget {
    fn from(phid: PhidgetHandle) -> Self {
        Self::new(phid)
    }
}
