// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::{Result, ReturnCode};
use phidget_sys::{self as ffi, PhidgetHandle};
use std::{os::raw::c_int, time::Duration};

pub trait Phidget {
    fn as_handle(&mut self) -> PhidgetHandle;

    /// Attempt to open the humidity sensor for input.
    fn open(&mut self) -> Result<()> {
        ReturnCode::result(unsafe { ffi::Phidget_open(self.as_handle()) })
    }

    fn open_wait(&mut self, to: Duration) -> Result<()> {
        let ms = to.as_millis() as u32;
        ReturnCode::result(unsafe { ffi::Phidget_openWaitForAttachment(self.as_handle(), ms) })
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

    // ----- Filters -----

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
