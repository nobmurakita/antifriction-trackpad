use crate::framework::core_foundation::CFMutableArrayRef;
use std::os::raw::{c_int, c_float, c_double, c_void};

pub type MTDeviceRef = *const c_void;

#[repr(C)]
pub struct mtPoint {
    pub x: c_double,
    pub y: c_double,
}

#[repr(C)]
pub struct mtReadout {
    pub position: mtPoint,
    pub velocity: mtPoint,
}

#[repr(C)]
pub struct Finger {
    pub frame: c_int,
    pub timestamp: c_double,
    pub identifier: c_int,
    pub state: c_int,
    pub unknown1: c_int,
    pub unknown2: c_int,
    pub normalized: mtReadout,
    pub size: c_float,
    pub unknown3: c_int,
    pub angle: c_float,
    pub major_axis: c_float,
    pub minor_axis: c_float,
    pub unknown4: mtReadout,
    pub unknown5_1: c_int,
    pub unknown5_2: c_int,
    pub unknown6: c_float,
}

pub type MTContactCallbackFunction = extern fn(
    MTDeviceRef,
    *const Finger,
    c_int,
    c_double,
    c_int
) -> c_int;

#[link(name = "MultitouchSupport", kind = "framework")]
extern "C" {
    pub fn MTDeviceCreateList() -> CFMutableArrayRef;

    pub fn MTRegisterContactFrameCallback(
        device: MTDeviceRef,
        callback: MTContactCallbackFunction
    );

    pub fn MTUnregisterContactFrameCallback(
        device: MTDeviceRef,
        callback: MTContactCallbackFunction
    );

    pub fn MTDeviceStart(device: MTDeviceRef, _: c_int);

    pub fn MTDeviceStop(device: MTDeviceRef, _: c_int);
}
