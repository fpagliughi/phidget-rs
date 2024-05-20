/// Phidget hub
pub mod hub;
pub use crate::devices::hub::{Hub, HubPortMode};

/// Phidget hmidity sensor
pub mod humidity_sensor;
pub use crate::devices::humidity_sensor::HumiditySensor;

/// Phidget stepper
pub mod stepper;
pub use crate::devices::stepper::Stepper;

/// Phidget temerature sensor
pub mod temperature_sensor;
pub use crate::devices::temperature_sensor::TemperatureSensor;

/// Phidget digital input
pub mod digital_output;
pub use crate::devices::digital_input::DigitalInput;

/// Phidget digital ouput
pub mod digital_input;
pub use crate::devices::digital_output::DigitalOutput;

/// Phidget voltage input
pub mod voltage_input;
pub use crate::devices::voltage_input::VoltageInput;

/// Phidget voltage ratio input
pub mod voltage_ratio_input;
pub use crate::devices::voltage_ratio_input::VoltageRatioInput;

/// Phidget voltage ouput
pub mod voltage_output;
// mod voltage_ratio_input;

pub use crate::devices::voltage_output::VoltageOutput;
