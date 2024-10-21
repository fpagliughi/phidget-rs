// phidget-rs/examples/servermon.rs
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

//! Rust Phidget example application to monitor server additions and
//! removals.
//!
//! This assumes a server running on localhost.
//!
//! After starting this monitor, try disconnecting or shutting down the
//! server then restarting it.
//!
//! Note that on Linx, this seems to require superuser privilege to detect
//! server "add" event. And it uses significant CPU resources.

use phidget::net::{Server as PhidgetServer, ServerType as PhidgetServerType};
use std::{process, thread};

// Handler for when a server was added
fn on_server_added(srvr: PhidgetServer) {
    println!("Server added: {:?}", srvr);
}

// Handler for when a server was removed
fn on_server_removed(srvr: PhidgetServer) {
    println!("Server removed: {:?}", srvr);
}

fn main() -> anyhow::Result<()> {
    println!("Phidget server monitor.\n");

    // Register the handlers
    phidget::net::set_on_server_added_handler(on_server_added)?;
    phidget::net::set_on_server_removed_handler(on_server_removed)?;

    // Start the discovery
    if let Err(err) = phidget::net::enable_server_discovery(PhidgetServerType::DeviceRemote) {
        eprintln!("{}", err);
        process::exit(1);
    }

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
