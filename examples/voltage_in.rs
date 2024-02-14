// phidget-rs/examples/voltage_in.rs
//
// Copyright (c) 2023, Frank Pagliughi
//
// This file is an example application for the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Rust Phidget example application to read voltage input values.

use phidget::Phidget;
use std::{thread, time::Duration};

// The open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    println!("Phidgets-rs {VERSION}");

    let port = 0; // Use a specific port on a VINT hub directly
    let serial = 0; // Specify the serial number of the device to open
    let channel = 0; // Specify the channel number of the device to open


    let use_hub = true;

    println!("Opening Phidget voltage input device...");
    let mut vin = phidget::devices::VoltageInput::new();

    // Whether we should use a hub port directly as the input,
    // and if so, which one?
    vin.set_is_hub_port_device(use_hub)?;
    vin.set_hub_port(port)?;
    vin.set_serial_number(serial)?;
    vin.set_channel(channel)?;

    vin.open_wait(TIMEOUT)?;

    if use_hub {
        let port = vin.hub_port()?;
        println!("Opened on hub port: {}", port);
    }

    let v = vin.voltage()?;
    println!("Voltage: {}", v);

    vin.set_on_voltage_change_handler(|_, v: f64| {
        println!("Voltage: {}", v);
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
