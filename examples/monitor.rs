// phidget-rs/examples/MONITOR.rs
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

//! Rust Phidget example application to monitor device attach/detach
//! (hotplug) events.
//!

use clap::{arg, ArgAction};
use phidget::{manager::PhidgetManager, Phidget};
use std::thread;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let _opts = clap::Command::new("monitor")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Device Monitor Example")
        .disable_help_flag(true)
        .arg(
            arg!(--help "Print help information")
                .short('?')
                .action(ArgAction::Help),
        )
        .get_matches();

    println!("Opening Phidget device manager...");
    let mut mgr = PhidgetManager::new();

    mgr.open()?;
    println!("\nWaiting on events. Hit ^C to exit.");

    // Set callback handlers
    mgr.set_on_attach_handler(|dev| {
        println!("Attach");
        if dev.is_hub_port_device().unwrap() {
            println!("  Hub device");
        }
        println!(
            "  Device Class: {:?} [{}]",
            dev.device_class().unwrap(),
            dev.device_class_name().unwrap()
        );
        println!("  Channel Name: {}", dev.channel_name().unwrap());
        println!("  Channel Class: {}", dev.channel_class_name().unwrap());
        println!("  Hub Port: {}", dev.hub_port().unwrap_or(-1));
        println!("  SKU: {}", dev.device_sku().unwrap());
    })?;

    mgr.set_on_detach_handler(|_| {
        println!("Detach");
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
