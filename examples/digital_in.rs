// phidget-rs/examples/digital_in.rs
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

//! Rust Phidget example application to read digital input values.
//!
//! The input bit can be selected by choosing the serial number of a device
//! and channel number for the bit.
//!
//! You can also use a port on a hub as a digital intput. In that case the
//! port is active low (true when grounded) as measured between the white
//! and black lines. Select the hub (-h) option and the port number, like:
//!
//! ```text
//! $ digital_in -h -p 5
//! ```

use clap::{arg, value_parser, ArgAction};
use phidget::{devices::DigitalInput, Phidget};
use std::{thread, time::Duration};

// The open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    println!("Phidgets-rs {VERSION}");

    let use_hub = true;

    let port = 0; // Use a specific port on a VINT hub directly
    let serial = 0; // Specify the serial number of the device to open
    let channel = 0; // Specify the channel number of the device to open

    println!("Opening Phidget digital input device...");
    let mut digin = DigitalInput::new();

    // Whether we should use a hub port directly as the input,
    // and if so, which one?
    digin.set_is_hub_port_device(use_hub)?;
    digin.set_hub_port(port)?;
    digin.set_serial_number(serial)?;
    digin.set_channel(channel)?;

    digin.open_wait(TIMEOUT)?;

    if use_hub {
        let port = digin.hub_port()?;
        println!("Opened on hub port: {}", port);
    }

    let s = digin.get_state()?;
    println!("Digital: {}", s);

    digin.set_on_state_change_handler(|_, s: i32| {
        println!("State: {}", s);
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
