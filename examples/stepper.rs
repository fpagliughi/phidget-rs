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
use clap::{arg, value_parser, ArgAction};
use phidget::Phidget;
use std::{thread, time::Duration};
// The open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let opts = clap::Command::new("digital_in")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Digital Input Example")
        .disable_help_flag(true)
        .arg(
            arg!(--help "Print help information")
                .short('?')
                .action(ArgAction::Help),
        )
        .arg(
            arg!(-s --serial [serial] "Specify the serial number of the device to open")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(-c --channel [channel] "Specify the channel number of the device to open")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(-p --port [port] "Use a specific port on a VINT hub directly")
                .value_parser(value_parser!(i32)),
        )
        .arg(arg!(-t --target [target] "Set target position").value_parser(value_parser!(f64)))
        .arg(arg!(-h --hub "Use a hub VINT input port directly").action(ArgAction::SetTrue))
        .get_matches();

    let use_hub = opts.get_flag("hub");

    println!("Opening Phidget stepper device...");
    let mut stepper = phidget::devices::Stepper::new();

    // Whether we should use a hub port directly as the input,
    // and if so, which one?
    stepper.set_is_hub_port_device(use_hub)?;
    if let Some(&port) = opts.get_one::<i32>("port") {
        stepper.set_hub_port(port)?;
    }

    // Some other device selection filters...
    if let Some(&serial) = opts.get_one::<i32>("serial") {
        stepper.set_serial_number(serial)?;
    }

    if let Some(&channel) = opts.get_one::<i32>("channel") {
        stepper.set_channel(channel)?;
    }
    let mut target_position = 0f64;
    if let Some(&target) = opts.get_one::<f64>("target") {
        target_position = target;
    }

    stepper.open_wait(TIMEOUT)?;

    let port = stepper.hub_port()?;
    println!("Opened on hub port: {}", port);

    let position = stepper.position()?;
    println!("Stepper position: {}", position);

    stepper.set_on_position_change_handler(|_, position: f64| {
        println!("Stepper position: {}", position);
    })?;

    stepper.set_target_position(target_position)?;

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
