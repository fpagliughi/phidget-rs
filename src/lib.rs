// phidget-rs/src/lib.rs
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

//! Safe Rust bindings to the phidget22 library.
//!
//! # Basic usage
//!
//! This example shows how to access a simple Digital Input, connected to the first available channel of a Vint HUB.
//! See the `examples` directory for more thorough code snippets.
//! ```rust,no_run
//! use phidget::{DigitalOutput, Phidget};
//! # use std::time::Duration;
//!
//! // Create a handle to a Digital Output device
//! let mut out = DigitalOutput::new();
//! // Before opening the device, set its VINT hub port
//! out.set_is_hub_port_device(true).unwrap();
//! out.set_hub_port(0).unwrap();
//!
//! // Start connection. Make sure to handle the result
//! // to check the device is available
//! out.open_wait_default().unwrap();
//!
//! // Control the output device
//! loop {
//!     println!("Turn on LED");
//!     out.set_state(1).unwrap();
//!     std::thread::sleep(Duration::from_secs(3));
//!
//!     println!("Turn off LED");
//!     out.set_state(0).unwrap();
//!     std::thread::sleep(Duration::from_secs(3));
//! }
//! ```
//!
//! # Callbacks
//! In order to activate an output phidget device depending on the state of other sensors,
//! for instance by turning on an LED whenever another sensor detects something,
//! you need to set a callback listening for sensor value changes, and keep a valid handle to the output device to set its state.
//!
//! The problem is, Phidget callbacks do run in a different thread. A Phidget handle can already be sent
//! to a different thread, as it implements [Send], but it doesn't implement [Sync].
//! Hence, if you desire to access the same handle from different callbacks, it has to be wrapped in a
//! Sync container, such as a [Mutex](std::sync::Mutex).
//!
//! ```rust,no_run
//! # use phidget::{Phidget, DigitalOutput, DigitalInput};
//! # use std::sync::Mutex;
//! # fn main()
//! # {
//! #    // Open a digitalInput to detect a button
//!     let mut button = DigitalInput::new();
//! #   button.set_channel(0).unwrap();
//!     // Open the digital output where
//!     // a LED is connected to.
//!     // In this example, it is initialized
//!     // and wrapped in a Mutex
//!     let led = Mutex::new({
//!         let mut tmp = DigitalOutput::new();
//!         tmp.set_channel(1).unwrap();
//!         tmp.open_wait_default().unwrap();
//!         tmp
//!     });
//!
//!     // Make the button alternate the LED state
//!     button.set_on_state_change_handler(move |_, s: u8| {
//!         let lock = led.lock().unwrap();
//!         match s {
//!             // Access the device inside the Mutex and change its state
//!             0 => lock.set_state(0).unwrap(),
//!             _ => lock.set_state(1).unwrap()
//!         }
//!     }).unwrap();
//! # }
//! ```

// Platform dependent whether necessary
#![allow(clippy::unnecessary_cast)]
// Lints
#![deny(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::{
    ffi::CStr,
    os::raw::{c_char, c_uint, c_void},
    ptr,
    time::Duration,
};

pub use phidget_sys::{
    self as ffi, PHIDGET_CHANNEL_ANY, PHIDGET_HUBPORTSPEED_AUTO, PHIDGET_HUBPORT_ANY,
    PHIDGET_SERIALNUMBER_ANY, PHIDGET_TIMEOUT_DEFAULT, PHIDGET_TIMEOUT_INFINITE,
};

/// The error types for the crate
pub mod errors;
pub use crate::errors::*;

/// The main Phidget trait
pub mod phidget;
pub use crate::phidget::{AttachCallback, DetachCallback, GenericPhidget, Phidget};

/// Network API
pub mod net;
pub use crate::net::ServerType;

/// Module containing all implemented devices
pub mod devices;
pub mod manager;

// For v0.1.x compatibility, sensors available at the root
pub use crate::devices::{
    digital_input::DigitalInput, digital_output::DigitalOutput, hub::Hub,
    humidity_sensor::HumiditySensor, temperature_sensor::TemperatureSensor,
    voltage_input::VoltageInput, voltage_output::VoltageOutput,
    voltage_ratio_input::VoltageRatioInput,
};

/// An infinite timeout (wait forever)
pub const TIMEOUT_INFINITE: Duration = Duration::from_millis(PHIDGET_TIMEOUT_INFINITE as u64);

/// The default timeout for the library
pub const TIMEOUT_DEFAULT: Duration = Duration::from_millis(PHIDGET_TIMEOUT_DEFAULT as u64);

/////////////////////////////////////////////////////////////////////////////
/// Gets a string from a phidget22 call.
/// This can be any function that takes a pointer to a c-str as the lone
/// argument.
pub(crate) fn get_ffi_string<F>(mut f: F) -> Result<String>
where
    F: FnMut(*mut *const c_char) -> c_uint,
{
    unsafe {
        let mut ver: *const c_char = ptr::null_mut();
        ReturnCode::result(f(&mut ver))?;
        if ver.is_null() {
            return Err(ReturnCode::NoMemory);
        }
        let s = CStr::from_ptr(ver);
        Ok(s.to_string_lossy().into())
    }
}

/// Release the memory held in a double-boxed callback function/lambda.
pub(crate) fn drop_cb<P: ?Sized>(cb: Option<*mut c_void>) {
    if let Some(ctx) = cb {
        let _: Box<Box<P>> = unsafe { Box::from_raw(ctx as *mut _) };
    }
}

/////////////////////////////////////////////////////////////////////////////
// Types from the Phidget22 library

/// Phidget Device ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum DeviceId {
    Nothing = ffi::Phidget_DeviceID_PHIDID_NOTHING,
    Unknown = ffi::Phidget_DeviceID_PHIDID_UNKNOWN,
    DigitalInputPort = ffi::Phidget_DeviceID_PHIDID_DIGITALINPUT_PORT,
    DigitalOutputPort = ffi::Phidget_DeviceID_PHIDID_DIGITALOUTPUT_PORT,
    VoltageInputPort = ffi::Phidget_DeviceID_PHIDID_VOLTAGEINPUT_PORT,
    VoltageRatioInputPort = ffi::Phidget_DeviceID_PHIDID_VOLTAGERATIOINPUT_PORT,
    Dictionary = ffi::Phidget_DeviceID_PHIDID_DICTIONARY,
    Phidget1000 = ffi::Phidget_DeviceID_PHIDID_1000,
    Phidget1001 = ffi::Phidget_DeviceID_PHIDID_1001,
    Phidget1002 = ffi::Phidget_DeviceID_PHIDID_1002,
    Phidget1008 = ffi::Phidget_DeviceID_PHIDID_1008,
    Phidget1010_1013_1018_1019 = ffi::Phidget_DeviceID_PHIDID_1010_1013_1018_1019,
    Phidget1011 = ffi::Phidget_DeviceID_PHIDID_1011,
    Phidget1012 = ffi::Phidget_DeviceID_PHIDID_1012,
    Phidget1014 = ffi::Phidget_DeviceID_PHIDID_1014,
    Phidget1015 = ffi::Phidget_DeviceID_PHIDID_1015,
    Phidget1016 = ffi::Phidget_DeviceID_PHIDID_1016,
    Phidget1017 = ffi::Phidget_DeviceID_PHIDID_1017,
    Phidget1023 = ffi::Phidget_DeviceID_PHIDID_1023,
    Phidget1024 = ffi::Phidget_DeviceID_PHIDID_1024,
    Phidget1030 = ffi::Phidget_DeviceID_PHIDID_1030,
    Phidget1031 = ffi::Phidget_DeviceID_PHIDID_1031,
    Phidget1032 = ffi::Phidget_DeviceID_PHIDID_1032,
    Phidget1040 = ffi::Phidget_DeviceID_PHIDID_1040,
    Phidget1041 = ffi::Phidget_DeviceID_PHIDID_1041,
    Phidget1042 = ffi::Phidget_DeviceID_PHIDID_1042,
    Phidget1043 = ffi::Phidget_DeviceID_PHIDID_1043,
    Phidget1044 = ffi::Phidget_DeviceID_PHIDID_1044,
    Phidget1045 = ffi::Phidget_DeviceID_PHIDID_1045,
    Phidget1046 = ffi::Phidget_DeviceID_PHIDID_1046,
    Phidget1047 = ffi::Phidget_DeviceID_PHIDID_1047,
    Phidget1048 = ffi::Phidget_DeviceID_PHIDID_1048,
    Phidget1049 = ffi::Phidget_DeviceID_PHIDID_1049,
    Phidget1051 = ffi::Phidget_DeviceID_PHIDID_1051,
    Phidget1052 = ffi::Phidget_DeviceID_PHIDID_1052,
    Phidget1053 = ffi::Phidget_DeviceID_PHIDID_1053,
    Phidget1054 = ffi::Phidget_DeviceID_PHIDID_1054,
    Phidget1055 = ffi::Phidget_DeviceID_PHIDID_1055,
    Phidget1056 = ffi::Phidget_DeviceID_PHIDID_1056,
    Phidget1057 = ffi::Phidget_DeviceID_PHIDID_1057,
    Phidget1058 = ffi::Phidget_DeviceID_PHIDID_1058,
    Phidget1059 = ffi::Phidget_DeviceID_PHIDID_1059,
    Phidget1060 = ffi::Phidget_DeviceID_PHIDID_1060,
    Phidget1061 = ffi::Phidget_DeviceID_PHIDID_1061,
    Phidget1062 = ffi::Phidget_DeviceID_PHIDID_1062,
    Phidget1063 = ffi::Phidget_DeviceID_PHIDID_1063,
    Phidget1064 = ffi::Phidget_DeviceID_PHIDID_1064,
    Phidget1065 = ffi::Phidget_DeviceID_PHIDID_1065,
    Phidget1066 = ffi::Phidget_DeviceID_PHIDID_1066,
    Phidget1067 = ffi::Phidget_DeviceID_PHIDID_1067,
    Phidget1202_1203 = ffi::Phidget_DeviceID_PHIDID_1202_1203,
    Phidget1204 = ffi::Phidget_DeviceID_PHIDID_1204,
    Phidget1215_1218 = ffi::Phidget_DeviceID_PHIDID_1215__1218,
    Phidget1219_1222 = ffi::Phidget_DeviceID_PHIDID_1219__1222,
    Adp1000 = ffi::Phidget_DeviceID_PHIDID_ADP1000,
    Daq1000 = ffi::Phidget_DeviceID_PHIDID_DAQ1000,
    Daq1200 = ffi::Phidget_DeviceID_PHIDID_DAQ1200,
    Daq1300 = ffi::Phidget_DeviceID_PHIDID_DAQ1300,
    Daq1301 = ffi::Phidget_DeviceID_PHIDID_DAQ1301,
    Daq1400 = ffi::Phidget_DeviceID_PHIDID_DAQ1400,
    Daq1500 = ffi::Phidget_DeviceID_PHIDID_DAQ1500,
    Dcc1000 = ffi::Phidget_DeviceID_PHIDID_DCC1000,
    Dcc1001 = ffi::Phidget_DeviceID_PHIDID_DCC1001,
    Dcc1002 = ffi::Phidget_DeviceID_PHIDID_DCC1002,
    Dcc1003 = ffi::Phidget_DeviceID_PHIDID_DCC1003,
    Dcc1100 = ffi::Phidget_DeviceID_PHIDID_DCC1100,
    Dst1000 = ffi::Phidget_DeviceID_PHIDID_DST1000,
    Dst1001 = ffi::Phidget_DeviceID_PHIDID_DST1001,
    Dst1002 = ffi::Phidget_DeviceID_PHIDID_DST1002,
    Dst1200 = ffi::Phidget_DeviceID_PHIDID_DST1200,
    Enc1000 = ffi::Phidget_DeviceID_PHIDID_ENC1000,
    Enc1001 = ffi::Phidget_DeviceID_PHIDID_ENC1001,
    FirmwareUpgradeSpi = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_SPI,
    FirmwareUpgradeStm32f0 = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32F0,
    FirmwareUpgradeStm32f3 = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32F3,
    FirmwareUpgradeStm32g0 = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32G0,
    FirmwareUpgradeStm8s = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM8S,
    FirmwareUpgradeUsb = ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_USB,
    Hin1000 = ffi::Phidget_DeviceID_PHIDID_HIN1000,
    Hin1001 = ffi::Phidget_DeviceID_PHIDID_HIN1001,
    Hin1100 = ffi::Phidget_DeviceID_PHIDID_HIN1100,
    Hin1101 = ffi::Phidget_DeviceID_PHIDID_HIN1101,
    Hub0000 = ffi::Phidget_DeviceID_PHIDID_HUB0000,
    Hub0001 = ffi::Phidget_DeviceID_PHIDID_HUB0001,
    Hub0002 = ffi::Phidget_DeviceID_PHIDID_HUB0002,
    Hub0004 = ffi::Phidget_DeviceID_PHIDID_HUB0004,
    Hub0007 = ffi::Phidget_DeviceID_PHIDID_HUB0007,
    Hub5000 = ffi::Phidget_DeviceID_PHIDID_HUB5000,
    Hum1000 = ffi::Phidget_DeviceID_PHIDID_HUM1000,
    Hum1001 = ffi::Phidget_DeviceID_PHIDID_HUM1001,
    Hum1100 = ffi::Phidget_DeviceID_PHIDID_HUM1100,
    InterfaceKit4_8_8 = ffi::Phidget_DeviceID_PHIDID_INTERFACEKIT_4_8_8,
    Lcd1100 = ffi::Phidget_DeviceID_PHIDID_LCD1100,
    Led1000 = ffi::Phidget_DeviceID_PHIDID_LED1000,
    Lux1000 = ffi::Phidget_DeviceID_PHIDID_LUX1000,
    Mot0100 = ffi::Phidget_DeviceID_PHIDID_MOT0100,
    Mot0109 = ffi::Phidget_DeviceID_PHIDID_MOT0109,
    Mot0110 = ffi::Phidget_DeviceID_PHIDID_MOT0110,
    Mot1100 = ffi::Phidget_DeviceID_PHIDID_MOT1100,
    Mot1101 = ffi::Phidget_DeviceID_PHIDID_MOT1101,
    Mot1102 = ffi::Phidget_DeviceID_PHIDID_MOT1102,
    Out1000 = ffi::Phidget_DeviceID_PHIDID_OUT1000,
    Out1001 = ffi::Phidget_DeviceID_PHIDID_OUT1001,
    Out1002 = ffi::Phidget_DeviceID_PHIDID_OUT1002,
    Out1100 = ffi::Phidget_DeviceID_PHIDID_OUT1100,
    Pre1000 = ffi::Phidget_DeviceID_PHIDID_PRE1000,
    Rcc0004 = ffi::Phidget_DeviceID_PHIDID_RCC0004,
    Rcc1000 = ffi::Phidget_DeviceID_PHIDID_RCC1000,
    Rel1000 = ffi::Phidget_DeviceID_PHIDID_REL1000,
    Rel1100 = ffi::Phidget_DeviceID_PHIDID_REL1100,
    Rel1101 = ffi::Phidget_DeviceID_PHIDID_REL1101,
    Saf1000 = ffi::Phidget_DeviceID_PHIDID_SAF1000,
    Snd1000 = ffi::Phidget_DeviceID_PHIDID_SND1000,
    Stc1000 = ffi::Phidget_DeviceID_PHIDID_STC1000,
    Stc1001 = ffi::Phidget_DeviceID_PHIDID_STC1001,
    Stc1002 = ffi::Phidget_DeviceID_PHIDID_STC1002,
    Stc1003 = ffi::Phidget_DeviceID_PHIDID_STC1003,
    Stc1005 = ffi::Phidget_DeviceID_PHIDID_STC1005,
    Tmp1000 = ffi::Phidget_DeviceID_PHIDID_TMP1000,
    Tmp1100 = ffi::Phidget_DeviceID_PHIDID_TMP1100,
    Tmp1101 = ffi::Phidget_DeviceID_PHIDID_TMP1101,
    Tmp1200 = ffi::Phidget_DeviceID_PHIDID_TMP1200,
    Vcp1000 = ffi::Phidget_DeviceID_PHIDID_VCP1000,
    Vcp1001 = ffi::Phidget_DeviceID_PHIDID_VCP1001,
    Vcp1002 = ffi::Phidget_DeviceID_PHIDID_VCP1002,
    Vcp1100 = ffi::Phidget_DeviceID_PHIDID_VCP1100,
}

impl TryFrom<u32> for DeviceId {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use DeviceId::*;
        match val {
            ffi::Phidget_DeviceID_PHIDID_NOTHING => Ok(Nothing),
            ffi::Phidget_DeviceID_PHIDID_UNKNOWN => Ok(Unknown),
            ffi::Phidget_DeviceID_PHIDID_DIGITALINPUT_PORT => Ok(DigitalInputPort),
            ffi::Phidget_DeviceID_PHIDID_DIGITALOUTPUT_PORT => Ok(DigitalOutputPort),
            ffi::Phidget_DeviceID_PHIDID_VOLTAGEINPUT_PORT => Ok(VoltageInputPort),
            ffi::Phidget_DeviceID_PHIDID_VOLTAGERATIOINPUT_PORT => Ok(VoltageRatioInputPort),
            ffi::Phidget_DeviceID_PHIDID_DICTIONARY => Ok(Dictionary),
            ffi::Phidget_DeviceID_PHIDID_1000 => Ok(Phidget1000),
            ffi::Phidget_DeviceID_PHIDID_1001 => Ok(Phidget1001),
            ffi::Phidget_DeviceID_PHIDID_1002 => Ok(Phidget1002),
            ffi::Phidget_DeviceID_PHIDID_1008 => Ok(Phidget1008),
            ffi::Phidget_DeviceID_PHIDID_1010_1013_1018_1019 => Ok(Phidget1010_1013_1018_1019),
            ffi::Phidget_DeviceID_PHIDID_1011 => Ok(Phidget1011),
            ffi::Phidget_DeviceID_PHIDID_1012 => Ok(Phidget1012),
            ffi::Phidget_DeviceID_PHIDID_1014 => Ok(Phidget1014),
            ffi::Phidget_DeviceID_PHIDID_1015 => Ok(Phidget1015),
            ffi::Phidget_DeviceID_PHIDID_1016 => Ok(Phidget1016),
            ffi::Phidget_DeviceID_PHIDID_1017 => Ok(Phidget1017),
            ffi::Phidget_DeviceID_PHIDID_1023 => Ok(Phidget1023),
            ffi::Phidget_DeviceID_PHIDID_1024 => Ok(Phidget1024),
            ffi::Phidget_DeviceID_PHIDID_1030 => Ok(Phidget1030),
            ffi::Phidget_DeviceID_PHIDID_1031 => Ok(Phidget1031),
            ffi::Phidget_DeviceID_PHIDID_1032 => Ok(Phidget1032),
            ffi::Phidget_DeviceID_PHIDID_1040 => Ok(Phidget1040),
            ffi::Phidget_DeviceID_PHIDID_1041 => Ok(Phidget1041),
            ffi::Phidget_DeviceID_PHIDID_1042 => Ok(Phidget1042),
            ffi::Phidget_DeviceID_PHIDID_1043 => Ok(Phidget1043),
            ffi::Phidget_DeviceID_PHIDID_1044 => Ok(Phidget1044),
            ffi::Phidget_DeviceID_PHIDID_1045 => Ok(Phidget1045),
            ffi::Phidget_DeviceID_PHIDID_1046 => Ok(Phidget1046),
            ffi::Phidget_DeviceID_PHIDID_1047 => Ok(Phidget1047),
            ffi::Phidget_DeviceID_PHIDID_1048 => Ok(Phidget1048),
            ffi::Phidget_DeviceID_PHIDID_1049 => Ok(Phidget1049),
            ffi::Phidget_DeviceID_PHIDID_1051 => Ok(Phidget1051),
            ffi::Phidget_DeviceID_PHIDID_1052 => Ok(Phidget1052),
            ffi::Phidget_DeviceID_PHIDID_1053 => Ok(Phidget1053),
            ffi::Phidget_DeviceID_PHIDID_1054 => Ok(Phidget1054),
            ffi::Phidget_DeviceID_PHIDID_1055 => Ok(Phidget1055),
            ffi::Phidget_DeviceID_PHIDID_1056 => Ok(Phidget1056),
            ffi::Phidget_DeviceID_PHIDID_1057 => Ok(Phidget1057),
            ffi::Phidget_DeviceID_PHIDID_1058 => Ok(Phidget1058),
            ffi::Phidget_DeviceID_PHIDID_1059 => Ok(Phidget1059),
            ffi::Phidget_DeviceID_PHIDID_1060 => Ok(Phidget1060),
            ffi::Phidget_DeviceID_PHIDID_1061 => Ok(Phidget1061),
            ffi::Phidget_DeviceID_PHIDID_1062 => Ok(Phidget1062),
            ffi::Phidget_DeviceID_PHIDID_1063 => Ok(Phidget1063),
            ffi::Phidget_DeviceID_PHIDID_1064 => Ok(Phidget1064),
            ffi::Phidget_DeviceID_PHIDID_1065 => Ok(Phidget1065),
            ffi::Phidget_DeviceID_PHIDID_1066 => Ok(Phidget1066),
            ffi::Phidget_DeviceID_PHIDID_1067 => Ok(Phidget1067),
            ffi::Phidget_DeviceID_PHIDID_1202_1203 => Ok(Phidget1202_1203),
            ffi::Phidget_DeviceID_PHIDID_1204 => Ok(Phidget1204),
            ffi::Phidget_DeviceID_PHIDID_1215__1218 => Ok(Phidget1215_1218),
            ffi::Phidget_DeviceID_PHIDID_1219__1222 => Ok(Phidget1219_1222),
            ffi::Phidget_DeviceID_PHIDID_ADP1000 => Ok(Adp1000),
            ffi::Phidget_DeviceID_PHIDID_DAQ1000 => Ok(Daq1000),
            ffi::Phidget_DeviceID_PHIDID_DAQ1200 => Ok(Daq1200),
            ffi::Phidget_DeviceID_PHIDID_DAQ1300 => Ok(Daq1300),
            ffi::Phidget_DeviceID_PHIDID_DAQ1301 => Ok(Daq1301),
            ffi::Phidget_DeviceID_PHIDID_DAQ1400 => Ok(Daq1400),
            ffi::Phidget_DeviceID_PHIDID_DAQ1500 => Ok(Daq1500),
            ffi::Phidget_DeviceID_PHIDID_DCC1000 => Ok(Dcc1000),
            ffi::Phidget_DeviceID_PHIDID_DCC1001 => Ok(Dcc1001),
            ffi::Phidget_DeviceID_PHIDID_DCC1002 => Ok(Dcc1002),
            ffi::Phidget_DeviceID_PHIDID_DCC1003 => Ok(Dcc1003),
            ffi::Phidget_DeviceID_PHIDID_DCC1100 => Ok(Dcc1100),
            ffi::Phidget_DeviceID_PHIDID_DST1000 => Ok(Dst1000),
            ffi::Phidget_DeviceID_PHIDID_DST1001 => Ok(Dst1001),
            ffi::Phidget_DeviceID_PHIDID_DST1002 => Ok(Dst1002),
            ffi::Phidget_DeviceID_PHIDID_DST1200 => Ok(Dst1200),
            ffi::Phidget_DeviceID_PHIDID_ENC1000 => Ok(Enc1000),
            ffi::Phidget_DeviceID_PHIDID_ENC1001 => Ok(Enc1001),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_SPI => Ok(FirmwareUpgradeSpi),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32F0 => Ok(FirmwareUpgradeStm32f0),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32F3 => Ok(FirmwareUpgradeStm32f3),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM32G0 => Ok(FirmwareUpgradeStm32g0),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_STM8S => Ok(FirmwareUpgradeStm8s),
            ffi::Phidget_DeviceID_PHIDID_FIRMWARE_UPGRADE_USB => Ok(FirmwareUpgradeUsb),
            ffi::Phidget_DeviceID_PHIDID_HIN1000 => Ok(Hin1000),
            ffi::Phidget_DeviceID_PHIDID_HIN1001 => Ok(Hin1001),
            ffi::Phidget_DeviceID_PHIDID_HIN1100 => Ok(Hin1100),
            ffi::Phidget_DeviceID_PHIDID_HIN1101 => Ok(Hin1101),
            ffi::Phidget_DeviceID_PHIDID_HUB0000 => Ok(Hub0000),
            ffi::Phidget_DeviceID_PHIDID_HUB0001 => Ok(Hub0001),
            ffi::Phidget_DeviceID_PHIDID_HUB0002 => Ok(Hub0002),
            ffi::Phidget_DeviceID_PHIDID_HUB0004 => Ok(Hub0004),
            ffi::Phidget_DeviceID_PHIDID_HUB0007 => Ok(Hub0007),
            ffi::Phidget_DeviceID_PHIDID_HUB5000 => Ok(Hub5000),
            ffi::Phidget_DeviceID_PHIDID_HUM1000 => Ok(Hum1000),
            ffi::Phidget_DeviceID_PHIDID_HUM1001 => Ok(Hum1001),
            ffi::Phidget_DeviceID_PHIDID_HUM1100 => Ok(Hum1100),
            ffi::Phidget_DeviceID_PHIDID_INTERFACEKIT_4_8_8 => Ok(InterfaceKit4_8_8),
            ffi::Phidget_DeviceID_PHIDID_LCD1100 => Ok(Lcd1100),
            ffi::Phidget_DeviceID_PHIDID_LED1000 => Ok(Led1000),
            ffi::Phidget_DeviceID_PHIDID_LUX1000 => Ok(Lux1000),
            ffi::Phidget_DeviceID_PHIDID_MOT0100 => Ok(Mot0100),
            ffi::Phidget_DeviceID_PHIDID_MOT0109 => Ok(Mot0109),
            ffi::Phidget_DeviceID_PHIDID_MOT0110 => Ok(Mot0110),
            ffi::Phidget_DeviceID_PHIDID_MOT1100 => Ok(Mot1100),
            ffi::Phidget_DeviceID_PHIDID_MOT1101 => Ok(Mot1101),
            ffi::Phidget_DeviceID_PHIDID_MOT1102 => Ok(Mot1102),
            ffi::Phidget_DeviceID_PHIDID_OUT1000 => Ok(Out1000),
            ffi::Phidget_DeviceID_PHIDID_OUT1001 => Ok(Out1001),
            ffi::Phidget_DeviceID_PHIDID_OUT1002 => Ok(Out1002),
            ffi::Phidget_DeviceID_PHIDID_OUT1100 => Ok(Out1100),
            ffi::Phidget_DeviceID_PHIDID_PRE1000 => Ok(Pre1000),
            ffi::Phidget_DeviceID_PHIDID_RCC0004 => Ok(Rcc0004),
            ffi::Phidget_DeviceID_PHIDID_RCC1000 => Ok(Rcc1000),
            ffi::Phidget_DeviceID_PHIDID_REL1000 => Ok(Rel1000),
            ffi::Phidget_DeviceID_PHIDID_REL1100 => Ok(Rel1100),
            ffi::Phidget_DeviceID_PHIDID_REL1101 => Ok(Rel1101),
            ffi::Phidget_DeviceID_PHIDID_SAF1000 => Ok(Saf1000),
            ffi::Phidget_DeviceID_PHIDID_SND1000 => Ok(Snd1000),
            ffi::Phidget_DeviceID_PHIDID_STC1000 => Ok(Stc1000),
            ffi::Phidget_DeviceID_PHIDID_STC1001 => Ok(Stc1001),
            ffi::Phidget_DeviceID_PHIDID_STC1002 => Ok(Stc1002),
            ffi::Phidget_DeviceID_PHIDID_STC1003 => Ok(Stc1003),
            ffi::Phidget_DeviceID_PHIDID_STC1005 => Ok(Stc1005),
            ffi::Phidget_DeviceID_PHIDID_TMP1000 => Ok(Tmp1000),
            ffi::Phidget_DeviceID_PHIDID_TMP1100 => Ok(Tmp1100),
            ffi::Phidget_DeviceID_PHIDID_TMP1101 => Ok(Tmp1101),
            ffi::Phidget_DeviceID_PHIDID_TMP1200 => Ok(Tmp1200),
            ffi::Phidget_DeviceID_PHIDID_VCP1000 => Ok(Vcp1000),
            ffi::Phidget_DeviceID_PHIDID_VCP1001 => Ok(Vcp1001),
            ffi::Phidget_DeviceID_PHIDID_VCP1002 => Ok(Vcp1002),
            ffi::Phidget_DeviceID_PHIDID_VCP1100 => Ok(Vcp1100),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget channel class
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum ChannelClass {
    Nothing = ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING,
    Accelerometer = ffi::Phidget_ChannelClass_PHIDCHCLASS_ACCELEROMETER,
    BldcMotor = ffi::Phidget_ChannelClass_PHIDCHCLASS_BLDCMOTOR,
    CaptiveTouch = ffi::Phidget_ChannelClass_PHIDCHCLASS_CAPACITIVETOUCH,
    CurrentInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTINPUT,
    CurrentOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTOUTPUT,
    DataAdapter = ffi::Phidget_ChannelClass_PHIDCHCLASS_DATAADAPTER,
    DcMotor = ffi::Phidget_ChannelClass_PHIDCHCLASS_DCMOTOR,
    Dictionary = ffi::Phidget_ChannelClass_PHIDCHCLASS_DICTIONARY,
    DigitalInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALINPUT,
    DigitalOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALOUTPUT,
    DistanceSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_DISTANCESENSOR,
    Encoder = ffi::Phidget_ChannelClass_PHIDCHCLASS_ENCODER,
    FirmwareUpgrade = ffi::Phidget_ChannelClass_PHIDCHCLASS_FIRMWAREUPGRADE,
    FrequencyCounter = ffi::Phidget_ChannelClass_PHIDCHCLASS_FREQUENCYCOUNTER,
    Generic = ffi::Phidget_ChannelClass_PHIDCHCLASS_GENERIC,
    Gps = ffi::Phidget_ChannelClass_PHIDCHCLASS_GPS,
    Gyroscope = ffi::Phidget_ChannelClass_PHIDCHCLASS_GYROSCOPE,
    Hub = ffi::Phidget_ChannelClass_PHIDCHCLASS_HUB,
    HumiditySensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_HUMIDITYSENSOR,
    Ir = ffi::Phidget_ChannelClass_PHIDCHCLASS_IR,
    Lcd = ffi::Phidget_ChannelClass_PHIDCHCLASS_LCD,
    LightSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_LIGHTSENSOR,
    Magnetometer = ffi::Phidget_ChannelClass_PHIDCHCLASS_MAGNETOMETER,
    MeshDongle = ffi::Phidget_ChannelClass_PHIDCHCLASS_MESHDONGLE,
    MotorPositionController = ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORPOSITIONCONTROLLER,
    MotorVelocityController = ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORVELOCITYCONTROLLER,
    PhSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_PHSENSOR,
    PowerGuard = ffi::Phidget_ChannelClass_PHIDCHCLASS_POWERGUARD,
    PressureSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_PRESSURESENSOR,
    RcServo = ffi::Phidget_ChannelClass_PHIDCHCLASS_RCSERVO,
    ResistanceInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_RESISTANCEINPUT,
    Rfid = ffi::Phidget_ChannelClass_PHIDCHCLASS_RFID,
    SoundSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_SOUNDSENSOR,
    Spatial = ffi::Phidget_ChannelClass_PHIDCHCLASS_SPATIAL,
    Stepper = ffi::Phidget_ChannelClass_PHIDCHCLASS_STEPPER,
    TemperatureSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_TEMPERATURESENSOR,
    VoltageInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEINPUT,
    VoltageOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEOUTPUT,
    VoltageRatioInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGERATIOINPUT,
}

impl TryFrom<u32> for ChannelClass {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use ChannelClass::*;
        match val {
            ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING => Ok(Nothing),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_ACCELEROMETER => Ok(Accelerometer),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_BLDCMOTOR => Ok(BldcMotor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CAPACITIVETOUCH => Ok(CaptiveTouch),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTINPUT => Ok(CurrentInput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTOUTPUT => Ok(CurrentOutput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DATAADAPTER => Ok(DataAdapter),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DCMOTOR => Ok(DcMotor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DICTIONARY => Ok(Dictionary),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALINPUT => Ok(DigitalInput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALOUTPUT => Ok(DigitalOutput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DISTANCESENSOR => Ok(DistanceSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_ENCODER => Ok(Encoder),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_FIRMWAREUPGRADE => Ok(FirmwareUpgrade),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_FREQUENCYCOUNTER => Ok(FrequencyCounter),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GENERIC => Ok(Generic),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GPS => Ok(Gps),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GYROSCOPE => Ok(Gyroscope),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_HUB => Ok(Hub),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_HUMIDITYSENSOR => Ok(HumiditySensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_IR => Ok(Ir),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_LCD => Ok(Lcd),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_LIGHTSENSOR => Ok(LightSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MAGNETOMETER => Ok(Magnetometer),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MESHDONGLE => Ok(MeshDongle),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORPOSITIONCONTROLLER => {
                Ok(MotorPositionController)
            }
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORVELOCITYCONTROLLER => {
                Ok(MotorVelocityController)
            }
            ffi::Phidget_ChannelClass_PHIDCHCLASS_PHSENSOR => Ok(PhSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_POWERGUARD => Ok(PowerGuard),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_PRESSURESENSOR => Ok(PressureSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RCSERVO => Ok(RcServo),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RESISTANCEINPUT => Ok(ResistanceInput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RFID => Ok(Rfid),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_SOUNDSENSOR => Ok(SoundSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_SPATIAL => Ok(Spatial),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_STEPPER => Ok(Stepper),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_TEMPERATURESENSOR => Ok(TemperatureSensor),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEINPUT => Ok(VoltageInput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEOUTPUT => Ok(VoltageOutput),
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGERATIOINPUT => Ok(VoltageRatioInput),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/// Phidget device class
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum DeviceClass {
    Nothing = ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING,
    Accelerometer = ffi::Phidget_DeviceClass_PHIDCLASS_ACCELEROMETER,
    AdvancedServo = ffi::Phidget_DeviceClass_PHIDCLASS_ADVANCEDSERVO,
    Analog = ffi::Phidget_DeviceClass_PHIDCLASS_ANALOG,
    Bridge = ffi::Phidget_DeviceClass_PHIDCLASS_BRIDGE,
    DataAdapter = ffi::Phidget_DeviceClass_PHIDCLASS_DATAADAPTER,
    Dictionary = ffi::Phidget_DeviceClass_PHIDCLASS_DICTIONARY,
    Encoder = ffi::Phidget_DeviceClass_PHIDCLASS_ENCODER,
    FirmwareUpgrade = ffi::Phidget_DeviceClass_PHIDCLASS_FIRMWAREUPGRADE,
    FrequencyCounter = ffi::Phidget_DeviceClass_PHIDCLASS_FREQUENCYCOUNTER,
    Generic = ffi::Phidget_DeviceClass_PHIDCLASS_GENERIC,
    Gps = ffi::Phidget_DeviceClass_PHIDCLASS_GPS,
    Hub = ffi::Phidget_DeviceClass_PHIDCLASS_HUB,
    InterfaceKit = ffi::Phidget_DeviceClass_PHIDCLASS_INTERFACEKIT,
    Ir = ffi::Phidget_DeviceClass_PHIDCLASS_IR,
    Led = ffi::Phidget_DeviceClass_PHIDCLASS_LED,
    MeshDongle = ffi::Phidget_DeviceClass_PHIDCLASS_MESHDONGLE,
    MotorControl = ffi::Phidget_DeviceClass_PHIDCLASS_MOTORCONTROL,
    PhSensor = ffi::Phidget_DeviceClass_PHIDCLASS_PHSENSOR,
    Rfid = ffi::Phidget_DeviceClass_PHIDCLASS_RFID,
    Servo = ffi::Phidget_DeviceClass_PHIDCLASS_SERVO,
    Spatial = ffi::Phidget_DeviceClass_PHIDCLASS_SPATIAL,
    Steper = ffi::Phidget_DeviceClass_PHIDCLASS_STEPPER,
    TemperatreSensor = ffi::Phidget_DeviceClass_PHIDCLASS_TEMPERATURESENSOR,
    TextLcd = ffi::Phidget_DeviceClass_PHIDCLASS_TEXTLCD,
    Vint = ffi::Phidget_DeviceClass_PHIDCLASS_VINT,
}

impl TryFrom<u32> for DeviceClass {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use DeviceClass::*;
        match val {
            ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING => Ok(Nothing),
            ffi::Phidget_DeviceClass_PHIDCLASS_ACCELEROMETER => Ok(Accelerometer),
            ffi::Phidget_DeviceClass_PHIDCLASS_ADVANCEDSERVO => Ok(AdvancedServo),
            ffi::Phidget_DeviceClass_PHIDCLASS_ANALOG => Ok(Analog),
            ffi::Phidget_DeviceClass_PHIDCLASS_BRIDGE => Ok(Bridge),
            ffi::Phidget_DeviceClass_PHIDCLASS_DATAADAPTER => Ok(DataAdapter),
            ffi::Phidget_DeviceClass_PHIDCLASS_DICTIONARY => Ok(Dictionary),
            ffi::Phidget_DeviceClass_PHIDCLASS_ENCODER => Ok(Encoder),
            ffi::Phidget_DeviceClass_PHIDCLASS_FIRMWAREUPGRADE => Ok(FirmwareUpgrade),
            ffi::Phidget_DeviceClass_PHIDCLASS_FREQUENCYCOUNTER => Ok(FrequencyCounter),
            ffi::Phidget_DeviceClass_PHIDCLASS_GENERIC => Ok(Generic),
            ffi::Phidget_DeviceClass_PHIDCLASS_GPS => Ok(Gps),
            ffi::Phidget_DeviceClass_PHIDCLASS_HUB => Ok(Hub),
            ffi::Phidget_DeviceClass_PHIDCLASS_INTERFACEKIT => Ok(InterfaceKit),
            ffi::Phidget_DeviceClass_PHIDCLASS_IR => Ok(Ir),
            ffi::Phidget_DeviceClass_PHIDCLASS_LED => Ok(Led),
            ffi::Phidget_DeviceClass_PHIDCLASS_MESHDONGLE => Ok(MeshDongle),
            ffi::Phidget_DeviceClass_PHIDCLASS_MOTORCONTROL => Ok(MotorControl),
            ffi::Phidget_DeviceClass_PHIDCLASS_PHSENSOR => Ok(PhSensor),
            ffi::Phidget_DeviceClass_PHIDCLASS_RFID => Ok(Rfid),
            ffi::Phidget_DeviceClass_PHIDCLASS_SERVO => Ok(Servo),
            ffi::Phidget_DeviceClass_PHIDCLASS_SPATIAL => Ok(Spatial),
            ffi::Phidget_DeviceClass_PHIDCLASS_STEPPER => Ok(Steper),
            ffi::Phidget_DeviceClass_PHIDCLASS_TEMPERATURESENSOR => Ok(TemperatreSensor),
            ffi::Phidget_DeviceClass_PHIDCLASS_TEXTLCD => Ok(TextLcd),
            ffi::Phidget_DeviceClass_PHIDCLASS_VINT => Ok(Vint),
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// The the full version of the phidget22 library as a string.
/// This is something like, "Phidget22 - Version 1.14 - Built Mar 31 2023 22:44:59"
pub fn library_version() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersion(s) })
}

/// Gets just the version number of the phidget22 library as a string.
/// This is something like, "1.14"
pub fn library_version_number() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersionNumber(s) })
}

/// Closes all channels, and stops all threads. The library is reset to a
/// newly loaded state. All channel handles have been freed.
///
/// This function is intended for use in special cases where the library
/// cannot be unloaded between program runs, such as LabVIEW and Unity
/// Editor.
///
/// # Safety
///
/// This invalidates all Phidget objects that are running.
///
pub unsafe fn reset_library() -> Result<()> {
    ReturnCode::result(unsafe { ffi::Phidget_resetLibrary() })
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_queries() {
        assert!(library_version().is_ok());
        assert!(library_version_number().is_ok());
    }
}
