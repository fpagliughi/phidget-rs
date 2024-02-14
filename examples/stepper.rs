// phidget-rs/examples/temperature.rs
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

//! Rust Phidget example application to read temperature.
//!

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

    println!("Opening Phidget stepper...");
    let mut stepper = phidget::devices::Stepper::new();

    stepper.set_hub_port(port)?;
    stepper.set_serial_number(serial)?;
    stepper.set_channel(channel)?;

    stepper.open_wait(TIMEOUT)?;

    let port = stepper.hub_port()?;
    println!("Opened on hub port: {}", port);

    let position = stepper.get_position()?;
    println!("Stepper position: {}", position);

    stepper.set_on_position_change_handler(|_, position: f64| {
        println!("Stepper position: {}", position);
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
