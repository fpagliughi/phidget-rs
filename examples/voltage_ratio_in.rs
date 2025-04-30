// phidget-rs/examples/voltage_ratio_in.rs
//
// Copyright (c) 2023-2025, Frank Pagliughi
// Copyright (c) 2024 Jorge Guerra and Riley Hernandez
//
// This file is an example application for the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Rust Phidget example application to read voltage ratio input values.
//!
//! A number of sensors have a simple, linear, formula to get a value in
//! the form:
//!
//! ```text
//! value = (voltage_ratio - offset) * gain
//! ```
//! On a real application the offset and gain can be determined via a calibration procedure
//! (https://www.phidgets.com/docs/Calibrating_Load_Cells)
//! For this example, assume an offset of 2.5 and a gain (scaling factor) of 16 resulting
//! in a weight in grams. To see the output in grams, run this:
//!
//! ```text
//! $ voltage_in_ratio -o 2.5 -g 16.0
//! ```
//!
//! The input channel can be selected by choosing the serial number of a device
//! and channel number for the input.
//!

use clap::{arg, value_parser, ArgAction};
use phidget::{devices::VoltageRatioInput, Phidget};
use std::{thread, time::Duration};

// The open/connect timeout
// const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let opts = clap::Command::new("bridge_in")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Voltage (Analog) Input Example")
        .disable_help_flag(true)
        .arg(
            arg!(--help "Print help information")
                .short('?')
                .action(ArgAction::Help),
        )
        .arg(
            arg!(-s --serial [serial_num] "Specify the serial number of the device to open")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(-c --channel [chan] "Specify the channel number of the device to open")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(-p --port [port] "Use a specific port on a VINT hub directly")
                .value_parser(value_parser!(i32)),
        )
        .arg(arg!(-h --hub "Use a hub VINT input port directly").action(ArgAction::SetTrue))
        .arg(
            arg!(-o --offset [offset] "The offset for reading  [val = gain * (volt_ratio - offset)]")
                .default_value("0.0")
                .value_parser(value_parser!(f64)),
        )
        .arg(
            arg!(-g --gain [gain] "The gain for the reading [val = gain * (volts_ratio - offset)]")
                .default_value("1.0")
                .value_parser(value_parser!(f64)),
        )
        .get_matches();

    let use_hub = opts.get_flag("hub");

    println!("Opening Phidget bridge input device...");
    let mut vin = VoltageRatioInput::new();


    // Whether we should use a hub port directly as the input,
    // and if so, which one?
    vin.set_is_hub_port_device(use_hub)?;
    if let Some(&port) = opts.get_one::<i32>("port") {
        vin.set_hub_port(port)?;
    }

    // Some other device selection filters..
    if let Some(&num) = opts.get_one::<i32>("serial") {
        vin.set_serial_number(num)?;
    }

    if let Some(&chan) = opts.get_one::<i32>("channel") {
        vin.set_channel(chan)?;
    }

    let offset = *opts.get_one::<f64>("offset").unwrap();
    let gain = *opts.get_one::<f64>("gain").unwrap();

    vin.open_wait(TIMEOUT)?;

    if use_hub {
        let port = vin.hub_port()?;
        println!("Opened on hub port: {}", port);
    }

    // Set to the fastest supported sampling rate.
    let min_interval = vin.min_data_interval().unwrap();
    vin.set_data_interval(min_interval)?;

    println!("This device features a 2-3 second calibration procedure once first opened...");
    thread::sleep(Duration::from_millis(2000));
    println!("Calibration procedure complete");

    let v = vin.voltage_ratio()?;
    let val = (v - offset) * gain;
    println!("{:.4}", val);

    vin.set_on_voltage_ratio_change_handler(move |_, v| {
        let val = (v - offset) * gain;
        println!("{:.4}", val);
    })?;

    // ^C handler wakes up the main thread
    ctrlc::set_handler({
        let thr = thread::current();
        move || {
            println!("\nExiting...");
            thr.unpark();
        }
    })
    .expect("Error setting Ctrl-C handler");

    // Block until a ^C wakes us up to exit.
    thread::park();
    Ok(())
}
