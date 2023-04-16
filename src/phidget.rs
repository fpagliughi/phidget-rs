
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use crate::Result;
use phidget_sys::{self as ffi, PhidgetHandle};
use std::{time::Duration, os::raw::c_int};


pub trait Phidget {
    fn as_handle(&mut self) -> PhidgetHandle;

    /// Attempt to open the humidity sensor for input.
    fn open(&mut self) -> Result<()> {
        unsafe {
            crate::check_ret(ffi::Phidget_open(self.as_handle()))
        }
    }

    fn open_wait(&mut self, to: Duration) -> Result<()> {
        let ms = to.as_millis() as u32;
        unsafe {
            crate::check_ret(ffi::Phidget_openWaitForAttachment(self.as_handle(), ms))
        }
    }

    /// Closes the channel
    fn close(&mut self) -> Result<()> {
        unsafe {
            crate::check_ret(ffi::Phidget_close(self.as_handle()))
        }
    }

    /// Determines if the channel is open
    fn is_open(&mut self) -> Result<bool> {
        let mut open: c_int = 0;
        unsafe {
            crate::check_ret(ffi::Phidget_getIsOpen(self.as_handle(), &mut open))?;
        }
        Ok(open != 0)
    }

    /// Determines if the channel is open locally (not over a network).
    fn is_local(&mut self) -> Result<bool> {
        let mut local: c_int = 0;
        unsafe {
            crate::check_ret(ffi::Phidget_getIsLocal(self.as_handle(), &mut local))?;
        }
        Ok(local != 0)
    }

    /// Set true to open the channel locally (not over a network).
    fn set_local(&mut self, local: bool) -> Result<()> {
        let local = c_int::from(local);
        unsafe {
            crate::check_ret(ffi::Phidget_setIsLocal(self.as_handle(), local))
        }
    }

    /// Determines if the channel is open remotely (over a network).
    fn is_remote(&mut self) -> Result<bool> {
        let mut rem: c_int = 0;
        unsafe {
            crate::check_ret(ffi::Phidget_getIsRemote(self.as_handle(), &mut rem))?;
        }
        Ok(rem != 0)
    }

    /// Set true to open the channel locally,  (not over a network).
    fn set_remote(&mut self, rem: bool) -> Result<()> {
        let rem = c_int::from(rem);
        unsafe {
            crate::check_ret(ffi::Phidget_setIsRemote(self.as_handle(), rem))
        }
    }
}
