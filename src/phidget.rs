// phidget-rs/src/phidget.rs
//
// Copyright (c) 2025, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Phidget trait and `PhidgetRef` implementation.

use crate::{ChannelClass, DeviceClass, DeviceId, Error, Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle};
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
    ptr,
    time::Duration,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use std::ops::Not;

/// The signature for device attach callbacks
pub type AttachCallback = dyn Fn(&PhidgetRef) + Send + 'static;

/// The signature for device detach callbacks
pub type DetachCallback = dyn Fn(&PhidgetRef) + Send + 'static;

// Low-level, unsafe callback for device attach events
unsafe extern "C" fn on_attach(phid: PhidgetHandle, ctx: *mut c_void) {
    if !ctx.is_null() {
        let cb: &mut Box<AttachCallback> = &mut *(ctx as *mut _);
        let ph = PhidgetRef::from(phid);
        cb(&ph);
    }
}

// Low-level, unsafe callback for device detach events
unsafe extern "C" fn on_detach(phid: PhidgetHandle, ctx: *mut c_void) {
    if !ctx.is_null() {
        let cb: &mut Box<DetachCallback> = &mut *(ctx as *mut _);
        let ph = PhidgetRef::from(phid);
        cb(&ph);
    }
}

// ----- Callbacks -----

/// Assigns a handler that will be called when the Attach event occurs for
/// a matching phidget.
pub fn set_on_attach_handler<P, F>(ph: &mut P, cb: F) -> Result<*mut c_void>
where
    P: Phidget,
    F: Fn(&PhidgetRef) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<AttachCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::Phidget_setOnAttachHandler(ph.as_mut_handle(), Some(on_attach), ctx)
    })?;
    Ok(ctx)
}

/// Assigns a handler that will be called when the Detach event occurs for
/// a matching Phidget.
pub fn set_on_detach_handler<P, F>(ph: &mut P, cb: F) -> Result<*mut c_void>
where
    P: Phidget,
    F: Fn(&PhidgetRef) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<DetachCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::Phidget_setOnDetachHandler(ph.as_mut_handle(), Some(on_detach), ctx)
    })?;
    Ok(ctx)
}

/////////////////////////////////////////////////////////////////////////////

/// Information about a Phidget.
///
/// This information can be gathered piecemeal from any `Phidget`, or
/// collected all at once in one of this objects, which can also be
/// serialized if the `serde` trait is enabled.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PhidgetInfo {
    /// The channel name
    pub channel_name: String,
    /// The class of the channel
    pub channel_class: ChannelClass,
    /// The index of the channel on the device
    pub channel: i32,

    // TODO: subclass? dev chan count?
    /// The name of the device
    pub device_name: String,
    /// The class of the device
    pub device_class: DeviceClass,
    /// The Device ID
    pub device_id: DeviceId,
    /// The user-defined label stored in the device Flash (if any)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub device_label: Option<String>,
    /// The serial number of the device.
    /// If the device is part of a VINT, this is the serial number of the VINT hub.
    pub serial_number: i32,
    /// The hub port (if any)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "<&bool>::not"))]
    pub is_hub_port_device: bool,
    /// The hub port (if any)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hub_port: Option<i32>,
    /// The SKU (part number) of the Phidget
    pub device_sku: String,
}

/// Addressing properties to find Phidget.
///
#[derive(Default, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PhidgetFilter {
    // /// The class of the channel
    // #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    // pub channel_class: Option<ChannelClass>,
    /// The index of the channel on the device
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub channel: Option<i32>,
    /// The serial number of the device.
    /// If the device is part of a VINT, this is the serial number of the VINT hub.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub serial_number: Option<i32>,
    /// The hub port (if any)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub is_hub_port_device: Option<bool>,
    /// The hub port (if any)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hub_port: Option<i32>,
    /// The user-defined label stored in the device Flash
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub device_label: Option<String>,
}

impl From<PhidgetInfo> for PhidgetFilter {
    fn from(info: PhidgetInfo) -> Self {
        Self {
            //channel_class: Some(info.channel_class),
            channel: Some(info.channel),
            serial_number: Some(info.serial_number),
            is_hub_port_device: Some(info.is_hub_port_device),
            hub_port: info.hub_port,
            device_label: info.device_label,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// The base trait and implementation for Phidgets
pub trait Phidget {
    /// Get the immutable/shared phidget handle for the device.
    fn as_handle(&self) -> PhidgetHandle;

    /// Get the mutable phidget handle for the device
    fn as_mut_handle(&mut self) -> PhidgetHandle;

    /// Attempt to open the channel.
    fn open(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_open(self.as_mut_handle()) })
    }

    /// Attempt to open the channel, waiting a limited time
    /// for it to connect.
    /// The maximum time accepted is 49.7 days (i.e. 2^32 milliseconds)
    fn open_wait(&mut self, to: Duration) -> Result<()> {
        let ms = u32::try_from(to.as_millis()).map_err(|_| ReturnCode::InvalidArg)?;
        ReturnCode::result(unsafe { ffi::Phidget_openWaitForAttachment(self.as_mut_handle(), ms) })
    }

    /// Attempt to open the channel, waiting the default time
    /// for it to connect.
    fn open_wait_default(&mut self) -> Result<()> {
        self.open_wait(crate::TIMEOUT_DEFAULT)
    }

    /// Closes the channel
    fn close(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_close(self.as_mut_handle()) })
    }

    /// Determines if the channel is open
    fn is_open(&self) -> Result<bool> {
        let mut open: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsOpen(self.as_handle(), &mut open) })?;
        Ok(open != 0)
    }

    /// Determines if the channel is open and attached to a device.
    fn is_attached(&self) -> Result<bool> {
        let mut attached: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getAttached(self.as_handle(), &mut attached) })?;
        Ok(attached != 0)
    }

    /// Determines if the channel is open locally (not over a network).
    fn is_local(&self) -> Result<bool> {
        let mut local: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsLocal(self.as_handle(), &mut local) })?;
        Ok(local != 0)
    }

    /// Set true to open the channel locally (not over a network).
    fn set_local(&mut self, local: bool) -> Result<()> {
        let local = c_int::from(local);
        ReturnCode::result(unsafe { ffi::Phidget_setIsLocal(self.as_mut_handle(), local) })
    }

    /// Determines if the channel is open remotely (over a network).
    fn is_remote(&self) -> Result<bool> {
        let mut rem: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsRemote(self.as_handle(), &mut rem) })?;
        Ok(rem != 0)
    }

    /// Set true to open the channel locally,  (not over a network).
    fn set_remote(&mut self, rem: bool) -> Result<()> {
        let rem = c_int::from(rem);
        ReturnCode::result(unsafe { ffi::Phidget_setIsRemote(self.as_mut_handle(), rem) })
    }

    /// Gets the data interval for the device, if supported.
    fn data_interval(&self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms.into()))
    }

    /// Sets the data interval for the device, if supported.
    /// This value should be in the min/max data interval range.
    fn set_data_interval(&mut self, interval: Duration) -> Result<()> {
        let ms = u32::try_from(interval.as_millis()).map_err(|_| ReturnCode::InvalidArg)?;
        ReturnCode::result(unsafe { ffi::Phidget_setDataInterval(self.as_mut_handle(), ms) })
    }

    /// Gets the minimum data interval for the device, if supported.
    fn min_data_interval(&self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getMinDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms.into()))
    }

    /// Gets the maximum data interval for the device, if supported.
    fn max_data_interval(&self) -> Result<Duration> {
        let mut ms: u32 = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getMaxDataInterval(self.as_handle(), &mut ms) })?;
        Ok(Duration::from_millis(ms.into()))
    }

    /// Gets the data update rate for the device, if supported.
    fn data_rate(&self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Sets the data update rate for the device, if supported.
    fn set_data_rate(&mut self, freq: f64) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setDataRate(self.as_mut_handle(), freq) })
    }

    /// Gets the minimum data rate for the device, if supported.
    fn min_data_rate(&self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getMinDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Gets the maximum data rate for the device, if supported.
    fn max_data_rate(&self) -> Result<f64> {
        let mut freq: f64 = 0.0;
        ReturnCode::result(unsafe { ffi::Phidget_getMaxDataRate(self.as_handle(), &mut freq) })?;
        Ok(freq)
    }

    /// Get the number of channels of the specified class on the device.
    fn device_channel_count(&self, cls: ChannelClass) -> Result<u32> {
        let mut n: u32 = 0;
        let cls = cls as ffi::Phidget_ChannelClass;
        ReturnCode::result(unsafe {
            ffi::Phidget_getDeviceChannelCount(self.as_handle(), cls, &mut n)
        })?;
        Ok(n)
    }

    /// Gets class of the channel
    fn channel_class(&self) -> Result<ChannelClass> {
        let mut cls = ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING;
        ReturnCode::result(unsafe { ffi::Phidget_getChannelClass(self.as_handle(), &mut cls) })?;
        ChannelClass::try_from(cls)
    }

    /// Get the name of the channel class
    fn channel_class_name(&self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getChannelClassName(self.as_handle(), s) })
    }

    /// Get the channel's name.
    fn channel_name(&self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getChannelName(self.as_handle(), s) })
    }

    /// Gets class of the device
    fn device_class(&self) -> Result<DeviceClass> {
        let mut cls = ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING;
        ReturnCode::result(unsafe { ffi::Phidget_getDeviceClass(self.as_handle(), &mut cls) })?;
        DeviceClass::try_from(cls)
    }

    /// Get the name of the device class
    fn device_class_name(&self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getDeviceClassName(self.as_handle(), s) })
    }

    /// Gets the device ID
    fn device_id(&self) -> Result<DeviceId> {
        let mut id = ffi::Phidget_DeviceID_PHIDID_NOTHING;
        ReturnCode::result(unsafe { ffi::Phidget_getDeviceID(self.as_handle(), &mut id) })?;
        DeviceId::try_from(id)
    }

    /// Get the name of the device class
    fn device_name(&self) -> Result<String> {
        crate::get_ffi_string(|s| unsafe { ffi::Phidget_getDeviceName(self.as_handle(), s) })
    }

    /// Gets the full info for the phidget
    fn info(&self) -> Result<PhidgetInfo> {
        let info = PhidgetInfo {
            channel_name: self.channel_name()?,
            channel_class: self.channel_class()?,
            channel: self.channel()?,
            device_name: self.device_name()?,
            device_class: self.device_class()?,
            device_id: self.device_id()?,
            device_label: self.device_label()?,
            serial_number: self.serial_number()?,
            is_hub_port_device: self.is_hub_port_device()?,
            hub_port: self.hub_port().ok(),
            device_sku: self.device_sku()?,
        };
        Ok(info)
    }

    // ----- Filters -----

    /// Sets all the searchable parameters to find a device in one call.
    fn set_filter(&mut self, filter: &PhidgetFilter) -> Result<()> {
        if let Some(chan) = filter.channel {
            self.set_channel(chan)?;
        }
        if let Some(sn) = filter.serial_number {
            self.set_serial_number(sn)?;
        }
        if let Some(ihpd) = filter.is_hub_port_device {
            self.set_is_hub_port_device(ihpd)?;
        }
        if let Some(hub_port) = filter.hub_port {
            self.set_hub_port(hub_port)?;
        }
        if let Some(ref label) = filter.device_label {
            self.set_device_label(label)?;
        }
        Ok(())
    }

    /// Determines whether this channel is a VINT Hub port channel, or part
    /// of a VINT device attached to a hub port.
    fn is_hub_port_device(&self) -> Result<bool> {
        let mut on: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getIsHubPortDevice(self.as_handle(), &mut on) })?;
        Ok(on != 0)
    }

    /// Specify whether this channel should be opened on a VINT Hub port
    /// directly, or on a VINT device attached to a hub port.
    /// This must be set before the channel is opened.
    fn set_is_hub_port_device(&mut self, on: bool) -> Result<()> {
        let on = c_int::from(on);
        ReturnCode::result(unsafe { ffi::Phidget_setIsHubPortDevice(self.as_mut_handle(), on) })
    }

    /// Gets the index of the port on the VINT Hub to which the channel is attached.
    fn hub_port(&self) -> Result<i32> {
        let mut port: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getHubPort(self.as_handle(), &mut port) })?;
        Ok(port as i32)
    }

    /// Gets the index of the port on the VINT Hub to which the channel is attached.
    /// Set to PHIDGET_HUBPORT_ANY to open the channel on any port of the hub.
    /// This must be set before the channel is opened.
    fn set_hub_port(&mut self, port: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setHubPort(self.as_mut_handle(), port as c_int) })
    }

    /// Gets the index of the channel on the device.
    fn channel(&self) -> Result<i32> {
        let mut ch: c_int = 0;
        ReturnCode::result(unsafe { ffi::Phidget_getChannel(self.as_handle(), &mut ch) })?;
        Ok(ch as i32)
    }

    /// Sets the channel index to be opened.
    /// The default channel is 0. Set to PHIDGET_CHANNEL_ANY to open any
    /// channel on the specified device. This must be set before the channel
    /// is opened.
    fn set_channel(&mut self, chan: i32) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_setChannel(self.as_mut_handle(), chan as c_int) })
    }

    /// Gets the serial number of the device.
    /// If the channel is part of a VINT device, this is the serial number
    /// of the VINT Hub to which the device is attached.
    fn serial_number(&self) -> Result<i32> {
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
        ReturnCode::result(unsafe { ffi::Phidget_setDeviceSerialNumber(self.as_mut_handle(), sn) })
    }

    /// Gets the user-defined label that was burned into the device Flash
    /// memory.
    fn device_label(&self) -> Result<Option<String>> {
        unsafe {
            let mut sku: *const c_char = ptr::null();
            ReturnCode::result(ffi::Phidget_getDeviceLabel(self.as_handle(), &mut sku))?;
            Ok(match sku.is_null() {
                true => None,
                false => Some(CStr::from_ptr(sku).to_string_lossy().into_owned()),
            })
        }
    }

    /// Sets the user-defined label to use to search for the device.
    /// Note that this does not write a new label to the device's Flash memory.
    /// For that, see `Self::write_device_label()`
    fn set_device_label(&mut self, label: &str) -> Result<()> {
        let label = CString::new(label)?;
        ReturnCode::result(unsafe {
            ffi::Phidget_setDeviceLabel(self.as_mut_handle(), label.as_ptr())
        })
    }

    /// Write the user-defined label to the on-board Flash memory in the device.
    fn write_device_label(&mut self, label: &str) -> Result<()> {
        let label = CString::new(label)?;
        ReturnCode::result(unsafe {
            ffi::Phidget_writeDeviceLabel(self.as_mut_handle(), label.as_ptr())
        })
    }

    /// Gets the SKU (part number) of the Phidget to which the channel
    /// is attached.
    fn device_sku(&self) -> Result<String> {
        unsafe {
            let mut sku: *const c_char = ptr::null();
            ReturnCode::result(ffi::Phidget_getDeviceSKU(self.as_handle(), &mut sku))?;
            Ok(match sku.is_null() {
                true => String::new(),
                false => CStr::from_ptr(sku).to_string_lossy().into_owned(),
            })
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// A non-owning reference to a generic phidget.
///
/// This contains a wrapper around a generic PhidgetHandle, which might be
/// any type of device. It can be queried for additional information and
/// potentially converted into a specific device object.
///
/// This is a non-owning object. It will not release the underlying Phidget
/// when dropped. It is typically used to wrap a generic handle sent to a
/// callback from the phidget22 library.
#[allow(missing_copy_implementations)]
pub struct PhidgetRef(PhidgetHandle);

impl PhidgetRef {
    /// Creates a new, generic phidget for the handle.
    pub fn new(phid: PhidgetHandle) -> Self {
        Self(phid)
    }
}

impl Phidget for PhidgetRef {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.0
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.0
    }
}

impl From<PhidgetHandle> for PhidgetRef {
    fn from(phid: PhidgetHandle) -> Self {
        Self::new(phid)
    }
}

/////////////////////////////////////////////////////////////////////////////

/// A generic phidget.
///
/// This contains a wrapper around a generic PhidgetHandle, which might be
/// any type of device. It can be queried for additional information and
/// potentially converted into a specific device object.
///
/// Current this can only be created from a `PhidgetRef`, and will manage the
/// lifetime of the underlying `PhidgetHandle` using `ffi::Phidget_retain()`
/// and `ffi::Phidget_release()`.
#[allow(missing_copy_implementations)]
pub struct GenericPhidget(PhidgetHandle);

impl Phidget for GenericPhidget {
    fn as_mut_handle(&mut self) -> PhidgetHandle {
        self.0
    }
    fn as_handle(&self) -> PhidgetHandle {
        self.0
    }
}

unsafe impl Send for GenericPhidget {}

impl Drop for GenericPhidget {
    fn drop(&mut self) {
        if let Ok(true) = self.is_open() {
            let _ = self.close();
        }
        unsafe { ffi::Phidget_release(&mut self.0) };
    }
}

impl TryFrom<PhidgetRef> for GenericPhidget {
    type Error = Error;

    fn try_from(phid: PhidgetRef) -> Result<Self> {
        unsafe {
            ReturnCode::result(ffi::Phidget_retain(phid.0))?;
        }
        Ok(Self(phid.0))
    }
}
