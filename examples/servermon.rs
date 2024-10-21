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

//! Rust Phidget example application to moniter server additions.
//!

use phidget::net::Server as PhidgetServer;
use std::thread;

fn on_server_added(_srvr: PhidgetServer) {
    println!("Server added");
}

fn on_server_removed(_srvr: PhidgetServer) {
    println!("Server removed");
}

fn main() -> anyhow::Result<()> {
    phidget::net::set_on_server_added_handler(on_server_added)?;
    phidget::net::set_on_server_removed_handler(on_server_removed)?;

    if let Err(err) = phidget::net::add_server("local", "127.0.0.1", 5661, "", 0) {
        eprintln!("{}", err);
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
