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
use phidget::HubPortMode::DigitalInput;
// The open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    println!("Opening Phidget temperature sensor...");
    let mut sensor = phidget::St

    // Some device selection filters...
    if let Some(&port) = opts.get_one::<i32>("port") {
        sensor.set_hub_port(port)?;
    }

    if let Some(&num) = opts.get_one::<i32>("serial") {
        sensor.set_serial_number(num)?;
    }

    if let Some(&chan) = opts.get_one::<i32>("channel") {
        sensor.set_channel(chan)?;
    }

    sensor.open_wait(TIMEOUT)?;

    let port = sensor.hub_port()?;
    println!("Opened on hub port: {}", port);

    let t = sensor.temperature()?;
    println!("Temperature: {}", t);

    sensor.set_on_temperature_change_handler(|_, t: f64| {
        println!("Temperature: {}", t);
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
