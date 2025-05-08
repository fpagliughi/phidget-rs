// phidget-rs/examples/temperature.rs
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

//! Rust Phidget example application to read temperature.
//!

use clap::{arg, value_parser, ArgAction};
use phidget::{devices::TemperatureSensor, devices::ThermocoupleType, Phidget};
use std::{thread, time::Duration};

// The open/connect timeout
const TIMEOUT: Duration = Duration::from_secs(5);

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Convert Celsius to Fahrenheit
fn c_to_f(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let opts = clap::Command::new("temperature")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Temperature Monitoring Example")
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
            arg!(-t --type [tcType] "Set the thermocouple type [J|K|E|T]")
                .value_parser(["J", "K", "E", "T"])
                .value_parser(value_parser!(char)),
        )
        .arg(
            arg!(-i --interval [interval] "Sets the interval (period) for data collection, in ms")
                .default_value("1000")
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    println!("Opening Phidget temperature sensor...");
    let mut sensor = TemperatureSensor::new();

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

    // Determine which thermocouple type to use based on command line argument
    let tc_type = opts.get_one::<char>("type").map(|c| {
        use ThermocoupleType::*;
        match c {
            'J' => TypeJ,
            'K' => TypeK,
            'E' => TypeE,
            'T' => TypeT,
            _ => {
                eprintln!("Error: Unsupported thermocouple type '{c}'");
                std::process::exit(1);
            }
        }
    });

    if let Some(ref t) = tc_type {
        println!("Using thermocouple type: {:?}", t);
    }

    let interval = opts
        .get_one::<u32>("interval")
        .map(|&i| Duration::from_millis(i as u64))
        .unwrap();

    // When the sensor is attached, set some params
    sensor.set_on_attach_handler(move |sensor| {
        println!("\nTemperature sensor attached!");

        // Set the thermocouple type
        if let Some(tc_type) = tc_type {
            match sensor.set_thermocouple_type(tc_type) {
                Ok(_) => println!("Set thermocouple type to {:?}", tc_type),
                Err(err) => eprintln!("Failed to set thermocouple type: {}", err),
            }
        }

        // Set the acquisition interval (sampling period)
        if let Err(err) = sensor.set_data_interval(interval) {
            eprintln!("Error setting interval: {}", err);
        }
    })?;

    sensor.set_on_detach_handler(|_| {
        println!("Temperature sensor detached!");
    })?;

    // ...and/or set a callback handler
    sensor.set_on_temperature_change_handler(|_, t: f64| {
        println!("  {:.1}°C,  {:.1}°F", t, c_to_f(t));
    })?;

    // Open the device
    sensor.open_wait(TIMEOUT)?;

    let port = sensor.hub_port()?;
    println!("Opened on hub port: {}", port);
    println!("\nReading temperature. Hit ^C to exit.");

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
