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

// Looks like the latest Phidgets installer puts the framework into
// '/Library/Frameworks'
//
// Earlier versions sometimes used architecture-specific locations.
//
// But it also seems that the dynlib is in /usr/local/lib, so standard
// linking should work, too.
#[cfg(target_os = "macos")]
fn config_macos() {
    println!("cargo:rustc-link-lib=framework=Phidget22");
    println!(r"cargo:rustc-link-search=framework=/Library/Frameworks");

    let fw_path = if cfg!(target_arch = "x86_64") {
        "/usr/local"
    }
    else {
        "/opt/homebrew"
    };
    println!(r"cargo:rustc-link-search=framework={}/Frameworks", fw_path);
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
