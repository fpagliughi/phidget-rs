// phidget-rs/build.rs
//
// Copyright (c) 2023, Frank Pagliughi
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use std::env;

#[cfg(target_os = "macos")]
fn config_macos() {
    println!("cargo:rustc-link-lib=framework=phidget22");

    if cfg!(target_arch = "x86_64") {
        println!(r"cargo:rustc-link-search=framework=/usr/local/Frameworks/");
    }
    else {
        println!(r"cargo:rustc-link-search=framework=/opt/homebrew/Frameworks/");
    }
}

fn main() {
    // TODO: We should eventually find or regenerate the
    //      bindings file for the specific target.
    let tgt = env::var("TARGET").unwrap();
    println!("debug: Building for target: '{}'", tgt);

    // PHIDGET_ROOT should be set to point to the installation directory of phidgets
    // (e.g. C:\Program Files\Phidgets\Phidget22)
    if let Ok(phidget_libs) = env::var("PHIDGET_ROOT") {
        println!("cargo:rustc-link-search={}", phidget_libs);
    }

    #[cfg(target_os = "macos")]
    config_macos();

    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=phidget22");
}
