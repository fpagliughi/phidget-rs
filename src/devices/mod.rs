// phidget-rs/src/devices/mod.rs
//
// Copyright (c) 2023-2025, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

/// Phidget hub
pub mod hub;
pub use crate::devices::hub::{Hub, HubPortMode};

/// Phidget current input sensor
pub mod current_input;
pub use crate::devices::current_input::CurrentInput;

/// Phidget humidity sensor
pub mod humidity_sensor;
pub use crate::devices::humidity_sensor::{HumidityChangeCallback, HumiditySensor};

/// Phidget pressure sensor
pub mod pressure_sensor;
pub use crate::devices::pressure_sensor::{PressureChangeCallback, PressureSensor};

/// Phidget stepper
pub mod stepper;
pub use crate::devices::stepper::Stepper;

/// Phidget temperature sensor
pub mod temperature_sensor;
pub use crate::devices::temperature_sensor::{
    RtdType, RtdWireSetup, TemperatureChangeCallback, TemperatureSensor, ThermocoupleType,
};

/// Phidget digital input
pub mod digital_input;
pub use crate::devices::digital_input::{DigitalInput, InputMode, PowerSupply};

/// Phidget digital output
pub mod digital_output;
pub use crate::devices::digital_output::DigitalOutput;

/// Phidget voltage input
pub mod voltage_input;
pub use crate::devices::voltage_input::VoltageInput;

/// Phidget voltage ratio input
pub mod voltage_ratio_input;
pub use crate::devices::voltage_ratio_input::VoltageRatioInput;

/// Phidget voltage output
pub mod voltage_output;
pub use crate::devices::voltage_output::VoltageOutput;
