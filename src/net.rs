// phidget-rs/src/net.rs
//
// Copyright (c) 2023-2024, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//
//! Phidget network API
//!
//! This contains routines to attacth to remote Phidget servers to control
//! devices across a network,

use crate::{Error, Result, ReturnCode};
use phidget_sys as ffi;
use std::ffi::{c_char, c_int, c_void, CStr, CString};

/// Phidget server types
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum ServerType {
    #[default]
    None = ffi::PhidgetServerType_PHIDGETSERVER_NONE, // 0
    DeviceListener = ffi::PhidgetServerType_PHIDGETSERVER_DEVICELISTENER, // 1
    Device = ffi::PhidgetServerType_PHIDGETSERVER_DEVICE,                 // 2
    DeviceRemote = ffi::PhidgetServerType_PHIDGETSERVER_DEVICEREMOTE,     // 3
    WwwListener = ffi::PhidgetServerType_PHIDGETSERVER_WWWLISTENER,       // 4
    Www = ffi::PhidgetServerType_PHIDGETSERVER_WWW,                       // 5
    WwwRemote = ffi::PhidgetServerType_PHIDGETSERVER_WWWREMOTE,           // 6
    Sbc = ffi::PhidgetServerType_PHIDGETSERVER_SBC,                       // 7
}

impl TryFrom<u32> for ServerType {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        use ServerType::*;
        match val {
            ffi::PhidgetServerType_PHIDGETSERVER_NONE => Ok(None), // 0
            ffi::PhidgetServerType_PHIDGETSERVER_DEVICELISTENER => Ok(DeviceListener), // 1
            ffi::PhidgetServerType_PHIDGETSERVER_DEVICE => Ok(Device), // 2
            ffi::PhidgetServerType_PHIDGETSERVER_DEVICEREMOTE => Ok(DeviceRemote), // 3
            ffi::PhidgetServerType_PHIDGETSERVER_WWWLISTENER => Ok(WwwListener), // 4
            ffi::PhidgetServerType_PHIDGETSERVER_WWW => Ok(Www),   // 5
            ffi::PhidgetServerType_PHIDGETSERVER_WWWREMOTE => Ok(WwwRemote), // 6
            ffi::PhidgetServerType_PHIDGETSERVER_SBC => Ok(Sbc),   // 7
            _ => Err(ReturnCode::InvalidArg),
        }
    }
}

// Converts the C char ptr to a string, returning an empty string on
// a null ptr, or a UTF-8 error on conversion failure.
fn ptr_to_string(p: *const c_char) -> std::result::Result<String, std::str::Utf8Error> {
    let s = unsafe {
        match p.as_ref() {
            Some(s) => CStr::from_ptr(s).to_str()?.to_owned(),
            None => String::new(),
        }
    };
    Ok(s)
}

/// Information about a phidget server
#[derive(Default, Debug, Clone)]
pub struct Server {
    /// The server name
    pub name: String,
    /// The server type
    pub typ: ServerType,
    /// Flags
    pub flags: u32,
    /// The network address
    pub addr: String,
    /// The host name
    pub host: String,
    /// The port
    pub port: u16,
}

impl TryFrom<&ffi::PhidgetServer> for Server {
    type Error = Error;

    fn try_from(srvr: &ffi::PhidgetServer) -> Result<Self> {
        let name = ptr_to_string(srvr.name)?;
        let typ = ServerType::try_from(srvr.type_)?;
        let flags = srvr.flags as u32;
        let addr = ptr_to_string(srvr.addr)?;
        let host = ptr_to_string(srvr.host)?;
        let port = srvr.port as u16;

        Ok(Self {
            name,
            typ,
            flags,
            addr,
            host,
            port,
        })
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Register a server to which the client will try to connect.
///
/// This should be called when server discovery is not enabled or the
/// server is not discoverable.
pub fn add_server(server_name: &str, address: &str, port: i32, password: &str) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    let address = CString::new(address).unwrap();
    let password = CString::new(password).unwrap();
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_addServer(
            server_name.as_ptr(),
            address.as_ptr(),
            port as c_int,
            password.as_ptr(),
            0 as c_int, // 'flags' required to be zero
        )
    })
}

/// Removes the registration for a server.
///
/// If the client is currently connected to this server, the connection
/// will be closed.
pub fn remove_server(server_name: &str) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    ReturnCode::result(unsafe { ffi::PhidgetNet_removeServer(server_name.as_ptr()) })
}

/// Removes all registered servers.
pub fn remove_all_servers() -> Result<()> {
    ReturnCode::result(unsafe { ffi::PhidgetNet_removeAllServers() })
}

/// Enables attempts to connect to a discovered server, if attempts were
/// previously disabled by `disable_server()`.
pub fn enable_server(server_name: &str) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    ReturnCode::result(unsafe { ffi::PhidgetNet_enableServer(server_name.as_ptr()) })
}

/// Prevents attempts to automatically connect to a server.
pub fn disable_server(server_name: &str) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    ReturnCode::result(unsafe {
        // 'flags' param required to be zero
        ffi::PhidgetNet_disableServer(server_name.as_ptr(), 0 as c_int)
    })
}

/// Sets the password that will be used to attempt to connect to the server.
/// If the server has not already been added or discovered, a placeholder
/// server entry will be registered to use this password on the server once
/// it is discovered.
pub fn set_server_passward(server_name: &str, password: &str) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    let password = CString::new(password).unwrap();
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_setServerPassword(server_name.as_ptr(), password.as_ptr())
    })
}

/// Enables the dynamic discovery of servers that publish their identity to
/// the network.
/// Currently Multicast DNS is used to discover and publish Phidget servers.
pub fn enable_server_discovery(server_type: ServerType) -> Result<()> {
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_enableServerDiscovery(server_type as ffi::PhidgetServerType)
    })
}

/// Disables the dynamic discovery of servers that publish their identity.
/// This does not disconnect already established connections.
pub fn disable_server_discovery(server_type: ServerType) -> Result<()> {
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_disableServerDiscovery(server_type as ffi::PhidgetServerType)
    })
}

/// Callback when a server is added
pub type ServerAddedCallback = dyn Fn(Server) + Send + 'static;

/// Callback when a server is removed
pub type ServerRemovedCallback = dyn Fn(Server) + Send + 'static;

// Low-level, unsafe, callback for when a server is added
// The context is a double-boxed pointer to the safe Rust callback.
unsafe extern "C" fn on_server_added(
    ctx: *mut c_void,
    srvr: ffi::PhidgetServerHandle,
    // TODO: What is this?
    _kv: *mut c_void,
) {
    if ctx.is_null() {
        return;
    }

    let cb: &mut Box<ServerAddedCallback> = &mut *(ctx as *mut _);
    let srvr = srvr
        .as_ref()
        .and_then(|s| Server::try_from(s).ok())
        .unwrap_or_default();
    cb(srvr);
}

/// Assigns a handler to be called when a "server added" event occurs.
pub fn set_on_server_added_handler<F>(cb: F) -> Result<()>
where
    F: Fn(Server) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<ServerAddedCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::PhidgetNet_setOnServerAddedHandler(Some(on_server_added), ctx)
    })
}

// Low-level, unsafe, callback for when a server is removed
// The context is a double-boxed pointer to the safe Rust callback.
unsafe extern "C" fn on_server_removed(ctx: *mut c_void, srvr: ffi::PhidgetServerHandle) {
    if ctx.is_null() {
        return;
    }

    let cb: &mut Box<ServerRemovedCallback> = &mut *(ctx as *mut _);
    let srvr = srvr
        .as_ref()
        .and_then(|s| Server::try_from(s).ok())
        .unwrap_or_default();
    cb(srvr);
}

/// Assigns a handler to be called when a "server removed" event occurs.
pub fn set_on_server_removed_handler<F>(cb: F) -> Result<()>
where
    F: Fn(Server) + Send + 'static,
{
    // 1st box is fat ptr, 2nd is regular pointer.
    let cb: Box<Box<ServerRemovedCallback>> = Box::new(Box::new(cb));
    let ctx = Box::into_raw(cb) as *mut c_void;

    ReturnCode::result(unsafe {
        ffi::PhidgetNet_setOnServerRemovedHandler(Some(on_server_removed), ctx)
    })
}
