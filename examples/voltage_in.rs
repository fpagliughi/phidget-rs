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

use phidget::Phidget;
use std::{thread, time::Duration};

const TIMEOUT: Duration = Duration::from_millis(5000);

fn main() -> anyhow::Result<()> {
    //println!("{}", phidget::library_version()?);

    // TODO: Placeholder for a command-line param
    let use_hub = true;

    println!("Opening Phidget voltage input device...");
    let mut vin = phidget::VoltageInput::new();

    // Whether we should use the hub port directly as the input
    vin.set_is_hub_port_device(use_hub)?;

    vin.open_wait(TIMEOUT)?;
    let v = vin.voltage()?;
    println!("Voltage: {}", v);

    vin.set_on_voltage_change_handler(|_s: &phidget::VoltageInput, v: f64| {
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
