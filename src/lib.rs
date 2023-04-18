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

// Platform dependent whether necessary
#![allow(clippy::unnecessary_cast)]

use std::{
    ffi::CStr,
    os::raw::{c_char, c_uint},
    ptr,
};

pub use phidget_sys::{
    self as ffi, PHIDGET_CHANNEL_ANY, PHIDGET_HUBPORTSPEED_AUTO, PHIDGET_HUBPORT_ANY,
    PHIDGET_SERIALNUMBER_ANY, PHIDGET_TIMEOUT_DEFAULT, PHIDGET_TIMEOUT_INFINITE,
};

pub mod errors;
pub use crate::errors::*;

pub mod phidget;
pub use crate::phidget::Phidget;

pub mod humidity_sensor;
pub use crate::humidity_sensor::HumiditySensor;

pub mod temperature_sensor;
pub use crate::temperature_sensor::TemperatureSensor;

pub mod digital_io;
pub use crate::digital_io::{DigitalInput, DigitalOutput};

pub mod voltage_io;
pub use crate::voltage_io::{VoltageInput, VoltageOutput};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ChannelClass {
    Nothing = ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING, // 0
    Accelerometer = ffi::Phidget_ChannelClass_PHIDCHCLASS_ACCELEROMETER, // 1
    BldcMotor = ffi::Phidget_ChannelClass_PHIDCHCLASS_BLDCMOTOR, // 35
    CaptiveTouch = ffi::Phidget_ChannelClass_PHIDCHCLASS_CAPACITIVETOUCH, // 14
    CurrentInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTINPUT, // 2
    CurrentOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTOUTPUT, // 38
    DataAdapter = ffi::Phidget_ChannelClass_PHIDCHCLASS_DATAADAPTER, // 3
    DcMotor = ffi::Phidget_ChannelClass_PHIDCHCLASS_DCMOTOR, // 4
    Dictionary = ffi::Phidget_ChannelClass_PHIDCHCLASS_DICTIONARY, // 36
    DigitalInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALINPUT, // 5
    DigitalOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALOUTPUT, // 6
    DistanceSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_DISTANCESENSOR, // 7
    Encoder = ffi::Phidget_ChannelClass_PHIDCHCLASS_ENCODER, // 8
    FirmwareUpgrade = ffi::Phidget_ChannelClass_PHIDCHCLASS_FIRMWAREUPGRADE, // 32
    FrequencyCounter = ffi::Phidget_ChannelClass_PHIDCHCLASS_FREQUENCYCOUNTER, // 9
    Generic = ffi::Phidget_ChannelClass_PHIDCHCLASS_GENERIC, // 33
    Gps = ffi::Phidget_ChannelClass_PHIDCHCLASS_GPS,         // 10
    Gyroscope = ffi::Phidget_ChannelClass_PHIDCHCLASS_GYROSCOPE, // 12
    Hub = ffi::Phidget_ChannelClass_PHIDCHCLASS_HUB,         // 13
    HumiditySensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_HUMIDITYSENSOR, // 15
    Ir = ffi::Phidget_ChannelClass_PHIDCHCLASS_IR,           // 16
    Lcd = ffi::Phidget_ChannelClass_PHIDCHCLASS_LCD,         // 11
    LightSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_LIGHTSENSOR, // 17
    Magnetometer = ffi::Phidget_ChannelClass_PHIDCHCLASS_MAGNETOMETER, // 18
    MeshDongle = ffi::Phidget_ChannelClass_PHIDCHCLASS_MESHDONGLE, // 19
    MotorPositionController = ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORPOSITIONCONTROLLER, // 34
    MotorVelocityController = ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORVELOCITYCONTROLLER, // 39
    PhSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_PHSENSOR, // 37
    PowerGuard = ffi::Phidget_ChannelClass_PHIDCHCLASS_POWERGUARD, // = 20
    PressureSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_PRESSURESENSOR, // 21
    RcServo = ffi::Phidget_ChannelClass_PHIDCHCLASS_RCSERVO, // = 22
    ResistanceInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_RESISTANCEINPUT, // 23
    Rfid = ffi::Phidget_ChannelClass_PHIDCHCLASS_RFID,       // 24
    SoundSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_SOUNDSENSOR, // 25
    Spatial = ffi::Phidget_ChannelClass_PHIDCHCLASS_SPATIAL, // 26
    Stepper = ffi::Phidget_ChannelClass_PHIDCHCLASS_STEPPER, // 27
    TemperatureSensor = ffi::Phidget_ChannelClass_PHIDCHCLASS_TEMPERATURESENSOR, // 28
    VoltageInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEINPUT, // 29
    VoltageOutput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEOUTPUT, // 30
    VoltageRatioInput = ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGERATIOINPUT, // 31
}

impl TryFrom<u32> for ChannelClass {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use ChannelClass::*;
        match val {
            ffi::Phidget_ChannelClass_PHIDCHCLASS_NOTHING => Ok(Nothing), // 0
            ffi::Phidget_ChannelClass_PHIDCHCLASS_ACCELEROMETER => Ok(Accelerometer), // 1
            ffi::Phidget_ChannelClass_PHIDCHCLASS_BLDCMOTOR => Ok(BldcMotor), // 35
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CAPACITIVETOUCH => Ok(CaptiveTouch), // 14
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTINPUT => Ok(CurrentInput), // 2
            ffi::Phidget_ChannelClass_PHIDCHCLASS_CURRENTOUTPUT => Ok(CurrentOutput), // 38
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DATAADAPTER => Ok(DataAdapter), // 3
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DCMOTOR => Ok(DcMotor), // 4
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DICTIONARY => Ok(Dictionary), // 36
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALINPUT => Ok(DigitalInput), // 5
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DIGITALOUTPUT => Ok(DigitalOutput), // 6
            ffi::Phidget_ChannelClass_PHIDCHCLASS_DISTANCESENSOR => Ok(DistanceSensor), // 7
            ffi::Phidget_ChannelClass_PHIDCHCLASS_ENCODER => Ok(Encoder), // 8
            ffi::Phidget_ChannelClass_PHIDCHCLASS_FIRMWAREUPGRADE => Ok(FirmwareUpgrade), // 32
            ffi::Phidget_ChannelClass_PHIDCHCLASS_FREQUENCYCOUNTER => Ok(FrequencyCounter), // 9
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GENERIC => Ok(Generic), // 33
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GPS => Ok(Gps),         // 10
            ffi::Phidget_ChannelClass_PHIDCHCLASS_GYROSCOPE => Ok(Gyroscope), // 12
            ffi::Phidget_ChannelClass_PHIDCHCLASS_HUB => Ok(Hub),         // 13
            ffi::Phidget_ChannelClass_PHIDCHCLASS_HUMIDITYSENSOR => Ok(HumiditySensor), // 15
            ffi::Phidget_ChannelClass_PHIDCHCLASS_IR => Ok(Ir),           // 16
            ffi::Phidget_ChannelClass_PHIDCHCLASS_LCD => Ok(Lcd),         // 11
            ffi::Phidget_ChannelClass_PHIDCHCLASS_LIGHTSENSOR => Ok(LightSensor), // 17
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MAGNETOMETER => Ok(Magnetometer), // 18
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MESHDONGLE => Ok(MeshDongle), // 19
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORPOSITIONCONTROLLER => {
                Ok(MotorPositionController)
            } // 34
            ffi::Phidget_ChannelClass_PHIDCHCLASS_MOTORVELOCITYCONTROLLER => {
                Ok(MotorVelocityController)
            } // 39
            ffi::Phidget_ChannelClass_PHIDCHCLASS_PHSENSOR => Ok(PhSensor), // 37
            ffi::Phidget_ChannelClass_PHIDCHCLASS_POWERGUARD => Ok(PowerGuard), // = 20
            ffi::Phidget_ChannelClass_PHIDCHCLASS_PRESSURESENSOR => Ok(PressureSensor), // 21
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RCSERVO => Ok(RcServo), // = 22
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RESISTANCEINPUT => Ok(ResistanceInput), // 23
            ffi::Phidget_ChannelClass_PHIDCHCLASS_RFID => Ok(Rfid),       // 24
            ffi::Phidget_ChannelClass_PHIDCHCLASS_SOUNDSENSOR => Ok(SoundSensor), // 25
            ffi::Phidget_ChannelClass_PHIDCHCLASS_SPATIAL => Ok(Spatial), // 26
            ffi::Phidget_ChannelClass_PHIDCHCLASS_STEPPER => Ok(Stepper), // 27
            ffi::Phidget_ChannelClass_PHIDCHCLASS_TEMPERATURESENSOR => Ok(TemperatureSensor), // 28
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEINPUT => Ok(VoltageInput), // 29
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGEOUTPUT => Ok(VoltageOutput), // 30
            ffi::Phidget_ChannelClass_PHIDCHCLASS_VOLTAGERATIOINPUT => Ok(VoltageRatioInput), // 31
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DeviceClass {
    Nothing = ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING, // 0
    Accelerometer = ffi::Phidget_DeviceClass_PHIDCLASS_ACCELEROMETER, // 1
    AdvancedServo = ffi::Phidget_DeviceClass_PHIDCLASS_ADVANCEDSERVO, // 2
    Analog = ffi::Phidget_DeviceClass_PHIDCLASS_ANALOG,   // 3
    Bridge = ffi::Phidget_DeviceClass_PHIDCLASS_BRIDGE,   // 4
    DataAdapter = ffi::Phidget_DeviceClass_PHIDCLASS_DATAADAPTER, // 25
    Dictionary = ffi::Phidget_DeviceClass_PHIDCLASS_DICTIONARY, // 24
    Encoder = ffi::Phidget_DeviceClass_PHIDCLASS_ENCODER, // 5
    FirmwareUpgrade = ffi::Phidget_DeviceClass_PHIDCLASS_FIRMWAREUPGRADE, // 23
    FrequencyCounter = ffi::Phidget_DeviceClass_PHIDCLASS_FREQUENCYCOUNTER, // 6
    Generic = ffi::Phidget_DeviceClass_PHIDCLASS_GENERIC, // 22
    Gps = ffi::Phidget_DeviceClass_PHIDCLASS_GPS,         // 7
    Hub = ffi::Phidget_DeviceClass_PHIDCLASS_HUB,         // 8
    InterfaceKit = ffi::Phidget_DeviceClass_PHIDCLASS_INTERFACEKIT, // 9
    Ir = ffi::Phidget_DeviceClass_PHIDCLASS_IR,           // 10
    Led = ffi::Phidget_DeviceClass_PHIDCLASS_LED,         // 11
    MeshDongle = ffi::Phidget_DeviceClass_PHIDCLASS_MESHDONGLE, // 12
    MotorControl = ffi::Phidget_DeviceClass_PHIDCLASS_MOTORCONTROL, // 13
    PhSensor = ffi::Phidget_DeviceClass_PHIDCLASS_PHSENSOR, // 14
    Rfid = ffi::Phidget_DeviceClass_PHIDCLASS_RFID,       // 15
    Servo = ffi::Phidget_DeviceClass_PHIDCLASS_SERVO,     // 16
    Spatial = ffi::Phidget_DeviceClass_PHIDCLASS_SPATIAL, // 17
    Steper = ffi::Phidget_DeviceClass_PHIDCLASS_STEPPER,  // 18
    TemperatreSensor = ffi::Phidget_DeviceClass_PHIDCLASS_TEMPERATURESENSOR, // 19
    TextLcd = ffi::Phidget_DeviceClass_PHIDCLASS_TEXTLCD, // 20
    Vint = ffi::Phidget_DeviceClass_PHIDCLASS_VINT,       // 21
}

impl TryFrom<u32> for DeviceClass {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use DeviceClass::*;
        match val {
            ffi::Phidget_DeviceClass_PHIDCLASS_NOTHING => Ok(Nothing), // 0
            ffi::Phidget_DeviceClass_PHIDCLASS_ACCELEROMETER => Ok(Accelerometer), // 1
            ffi::Phidget_DeviceClass_PHIDCLASS_ADVANCEDSERVO => Ok(AdvancedServo), // 2
            ffi::Phidget_DeviceClass_PHIDCLASS_ANALOG => Ok(Analog),   // 3
            ffi::Phidget_DeviceClass_PHIDCLASS_BRIDGE => Ok(Bridge),   // 4
            ffi::Phidget_DeviceClass_PHIDCLASS_DATAADAPTER => Ok(DataAdapter), // 25
            ffi::Phidget_DeviceClass_PHIDCLASS_DICTIONARY => Ok(Dictionary), // 24
            ffi::Phidget_DeviceClass_PHIDCLASS_ENCODER => Ok(Encoder), // 5
            ffi::Phidget_DeviceClass_PHIDCLASS_FIRMWAREUPGRADE => Ok(FirmwareUpgrade), // 23
            ffi::Phidget_DeviceClass_PHIDCLASS_FREQUENCYCOUNTER => Ok(FrequencyCounter), // 6
            ffi::Phidget_DeviceClass_PHIDCLASS_GENERIC => Ok(Generic), // 22
            ffi::Phidget_DeviceClass_PHIDCLASS_GPS => Ok(Gps),         // 7
            ffi::Phidget_DeviceClass_PHIDCLASS_HUB => Ok(Hub),         // 8
            ffi::Phidget_DeviceClass_PHIDCLASS_INTERFACEKIT => Ok(InterfaceKit), // 9
            ffi::Phidget_DeviceClass_PHIDCLASS_IR => Ok(Ir),           // 10
            ffi::Phidget_DeviceClass_PHIDCLASS_LED => Ok(Led),         // 11
            ffi::Phidget_DeviceClass_PHIDCLASS_MESHDONGLE => Ok(MeshDongle), // 12
            ffi::Phidget_DeviceClass_PHIDCLASS_MOTORCONTROL => Ok(MotorControl), // 13
            ffi::Phidget_DeviceClass_PHIDCLASS_PHSENSOR => Ok(PhSensor), // 14
            ffi::Phidget_DeviceClass_PHIDCLASS_RFID => Ok(Rfid),       // 15
            ffi::Phidget_DeviceClass_PHIDCLASS_SERVO => Ok(Servo),     // 16
            ffi::Phidget_DeviceClass_PHIDCLASS_SPATIAL => Ok(Spatial), // 17
            ffi::Phidget_DeviceClass_PHIDCLASS_STEPPER => Ok(Steper),  // 18
            ffi::Phidget_DeviceClass_PHIDCLASS_TEMPERATURESENSOR => Ok(TemperatreSensor), // 19
            ffi::Phidget_DeviceClass_PHIDCLASS_TEXTLCD => Ok(TextLcd), // 20
            ffi::Phidget_DeviceClass_PHIDCLASS_VINT => Ok(Vint),       // 21
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}
/////////////////////////////////////////////////////////////////////////////

/// The the full version of the phidget22 library as a string.
/// This is somthing like, "Phidget22 - Version 1.14 - Built Mar 31 2023 22:44:59"
pub fn library_version() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersion(s) })
}

/// Gets just the version number of the phidget22 library as a string.
/// This is something like, "1.14"
pub fn library_version_number() -> Result<String> {
    get_ffi_string(|s| unsafe { ffi::Phidget_getLibraryVersionNumber(s) })
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
