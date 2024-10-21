// phidget-rs/examples/humidity.rs
//
// Copyright (c) 2023-2024, Frank Pagliughi
//
// This file is an example application for the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! Rust Phidget example application to read humidity.
//!

use clap::{arg, value_parser, ArgAction};
use phidget::{devices::HumiditySensor, Phidget};
use std::{thread, time::Duration};

// Open/connect timeout
const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let opts = clap::Command::new("humidity")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Humidity Monitoring Example")
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
        .arg(
            arg!(-i --interval [interval] "Sets the interval (period) for data collection, in ms")
                .default_value("1000")
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    println!("Opening Phidget humidity sensor...");
    let mut sensor = HumiditySensor::new();

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

    // Set the acquisition interval (sampling period)
    if let Some(&interval) = opts.get_one::<u32>("interval") {
        let dur = Duration::from_millis(interval as u64);
        if let Err(err) = sensor.set_data_interval(dur) {
            eprintln!("Error setting interval: {}", err);
        }
    }

    println!("\nReading humidity. Hit ^C to exit.");

    // Read a single value...
    let hum = sensor.humidity()?;
    println!("  {:.1}%", hum);

    // ...and/or set a callback handler
    sensor.set_on_humidity_change_handler(|_, hum: f64| {
        println!("  {:.1}%", hum);
    })?;

    // ^C handler wakes up the main thread to exit
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
