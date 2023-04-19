// phidget-rs/src/net.rs
//
// Copyright (c) 2023, Frank Pagliughi
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

use crate::{Error, Result, ReturnCode};
use phidget_sys as ffi;
use std::{
    ffi::CString,
    os::raw::c_int,
};

/// Phidget server types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum ServerType {
    None = ffi::PhidgetServerType_PHIDGETSERVER_NONE, // 0
    DeviceListener = ffi::PhidgetServerType_PHIDGETSERVER_DEVICELISTENER, // 1
    Device = ffi::PhidgetServerType_PHIDGETSERVER_DEVICE, // 2
    DeviceRemote = ffi::PhidgetServerType_PHIDGETSERVER_DEVICEREMOTE, // 3
    WwwListener = ffi::PhidgetServerType_PHIDGETSERVER_WWWLISTENER, // 4
    Www = ffi::PhidgetServerType_PHIDGETSERVER_WWW,   // 5
    WwwRemote = ffi::PhidgetServerType_PHIDGETSERVER_WWWREMOTE, // 6
    Sbc = ffi::PhidgetServerType_PHIDGETSERVER_SBC,   // 7
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

/////////////////////////////////////////////////////////////////////////////

/// Register a server to which the client will try to connect.
pub fn add_server(
    server_name: &str,
    address: &str,
    port: i32,
    password: &str,
    flags: i32,
) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    let address = CString::new(address).unwrap();
    let password = CString::new(password).unwrap();
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_addServer(
            server_name.as_ptr(),
            address.as_ptr(),
            port as c_int,
            password.as_ptr(),
            flags as c_int,
        )
    })
}

/// Removes the registration for a server.
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
pub fn disable_server(server_name: &str, flags: i32) -> Result<()> {
    let server_name = CString::new(server_name).unwrap();
    ReturnCode::result(unsafe {
        ffi::PhidgetNet_disableServer(server_name.as_ptr(), flags as c_int)
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
    ReturnCode::result(unsafe { ffi::PhidgetNet_enableServerDiscovery(server_type as u32) })
}

/// Disables the dynamic discovery of servers that publish their identity.
/// This does not disconnect already established connections.
pub fn disable_server_discovery(server_type: ServerType) -> Result<()> {
    ReturnCode::result(unsafe { ffi::PhidgetNet_disableServerDiscovery(server_type as u32) })
}

/*
pub type PhidgetNet_OnServerAddedCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        server: PhidgetServerHandle,
        kv: *mut ::std::os::raw::c_void,
    ),
>;
pub type PhidgetNet_OnServerRemovedCallback = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, server: PhidgetServerHandle),
>;
extern "C" {
    pub fn PhidgetNet_setOnServerAddedHandler(
        fptr: PhidgetNet_OnServerAddedCallback,
        ctx: *mut ::std::os::raw::c_void,
    ) -> PhidgetReturnCode;
}
extern "C" {
    pub fn PhidgetNet_setOnServerRemovedHandler(
        fptr: PhidgetNet_OnServerRemovedCallback,
        ctx: *mut ::std::os::raw::c_void,
    ) -> PhidgetReturnCode;
}
extern "C" {
    pub fn PhidgetNet_getServerAddressList(
        hostname: *const ::std::os::raw::c_char,
        addressFamily: ::std::os::raw::c_int,
        addressList: *mut *mut ::std::os::raw::c_char,
        count: *mut u32,
    ) -> PhidgetReturnCode;
}
extern "C" {
    pub fn PhidgetNet_freeServerAddressList(
        addressList: *mut *mut ::std::os::raw::c_char,
        count: u32,
    ) -> PhidgetReturnCode;
}
extern "C" {
    pub fn PhidgetNet_startServer(
        flags: ::std::os::raw::c_int,
        addressFamily: ::std::os::raw::c_int,
        serverName: *const ::std::os::raw::c_char,
        address: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        password: *const ::std::os::raw::c_char,
        server: *mut PhidgetServerHandle,
    ) -> PhidgetReturnCode;
}
extern "C" {
    pub fn PhidgetNet_stopServer(server: *mut PhidgetServerHandle) -> PhidgetReturnCode;
}
*/
