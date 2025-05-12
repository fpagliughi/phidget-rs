// phidget-rs/examples/digital_out.rs
//
// Copyright (c) 2025, Frank Pagliughi
//
// This file is an example application for the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Rust Phidget example application to write digital output values.
//!
//! The output bit can be selected by choosing the serial number of a device
//! and channel number for the bit.
//!
//! You can also use a port on a hub as a digital output. In that case the
//! port is active low (true when grounded) as measured between the white
//! and black lines. Select the hub (-h) option and the port number, like:
//!
//! ```text
//! $ digital_out -h -p 5
//! ```

use clap::{arg, value_parser, ArgAction};
use phidget::{devices::DigitalOutput, Phidget};
use std::{thread, time::Duration};

// The open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let opts = clap::Command::new("digital_out")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Digital Output Example")
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
        .arg(arg!(-h --hub "Use a hub VINT port directly").action(ArgAction::SetTrue))
        .arg(arg!(<state> "The new state of the bit. '0' or '1'").value_parser(value_parser!(u8)))
        .get_matches();

    let state = *opts.get_one::<u8>("state").unwrap();

    let use_hub = opts.get_flag("hub");

    println!("Opening Phidget digital output device...");
    let mut digout = DigitalOutput::new();

    // Whether we should use a hub port directly as the output,
    // and if so, which one?
    digout.set_is_hub_port_device(use_hub)?;
    if let Some(&port) = opts.get_one::<i32>("port") {
        digout.set_hub_port(port)?;
    }

    // Some other device selection filters...
    if let Some(&num) = opts.get_one::<i32>("serial") {
        digout.set_serial_number(num)?;
    }

    if let Some(&chan) = opts.get_one::<i32>("channel") {
        digout.set_channel(chan)?;
    }

    digout.open_wait(TIMEOUT)?;

    if use_hub {
        let port = digout.hub_port()?;
        println!("Opened on hub port: {}", port);
    }

    let s = digout.state()?;
    println!("Original State: {}", s);

    println!("Setting state to: {}", state);
    digout.set_state(state)?;

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
