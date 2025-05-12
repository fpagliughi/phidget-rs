// phidget-rs/examples/monitor.rs
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
use phidget::{manager::PhidgetManager, Phidget, PhidgetRef, Result};
use std::thread;

#[cfg(feature = "serde")]
use serde_json as json;

// The package version is used as the app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_dev_info(dev: &PhidgetRef) -> Result<()> {
    if let Ok(info) = dev.info() {
        if dev.is_hub_port_device().unwrap() {
            println!("  Hub device");
        }
        println!("  Device Class: {:?}", info.device_class);
        println!("  Channel Name: {}", info.channel_name);
        println!("  Channel Class: {:?}", info.channel_class /*_name*/);
        println!("  Hub Port: {}", dev.hub_port().unwrap_or(-1));
        println!("  SKU: {}", dev.device_sku().unwrap());
    }
    Ok(())
}

#[cfg(feature = "serde")]
fn print_dev_info_json(dev: &PhidgetRef) -> Result<()> {
    if let Ok(info) = dev.info() {
        println!("{}", json::to_string_pretty(&info).unwrap());
    }
    Ok(())
}

// --------------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    let cmd = clap::Command::new("monitor")
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Phidget Device Monitor Example")
        .disable_help_flag(true)
        .arg(
            arg!(-'?' --help "Print help information")
                .short('?')
                .action(ArgAction::Help),
        );

    #[cfg(feature = "serde")]
    let cmd = cmd.arg(arg!(-j --json "Output info as JSON").action(ArgAction::SetTrue));

    let _opts = cmd.get_matches();

    #[cfg(feature = "serde")]
    let use_json = _opts.is_present("json");

    println!("Opening Phidget device manager...");
    let mut mgr = PhidgetManager::new();

    mgr.open()?;
    println!("\nWaiting on events. Hit ^C to exit.");

    // Set callback handlers
    mgr.set_on_attach_handler(move |dev| {
        println!("Attach");
        #[cfg(feature = "serde")]
        if let Err(err) = if use_json {
            print_dev_info_json(&dev)
        }
        else {
            print_dev_info(&dev)
        } {
            println!("  Error retrieving device info: {}", err);
        }

        #[cfg(not(feature = "serde"))]
        if let Err(err) = print_dev_info(&dev) {
            println!("  Error retrieving device info: {}", err);
        }
    })?;

    mgr.set_on_detach_handler(move |dev| {
        println!("Detach");
        if let Err(err) = print_dev_info(&dev) {
            println!("  Error retrieving device info: {}", err);
        }
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
