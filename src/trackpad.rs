use crate::framework::core_foundation::*;
use crate::framework::multitouch_support::*;
use std::os::raw::{c_int, c_double};
use std::ptr;

static mut DEVICES: CFMutableArrayRef = ptr::null();

static mut CALLBACK: fn(bool, f64) = default_callback;

fn default_callback(touched: bool, timestamp: f64) {
    println!("{} {}", touched, timestamp);
}

extern "C" fn contact_frame_callback(
    _device: MTDeviceRef,
    data: *const Finger,
    data_num: c_int,
    timestamp: c_double,
    _frame: c_int
) -> c_int {
    unsafe {
        let touched = (0..data_num).any(|i| {
            (*data.offset(i as isize)).state == 4
        });
        CALLBACK(touched, timestamp);
    };
    0
}

pub fn set_callback(cb: fn(bool, f64)) {
    unsafe {
        CALLBACK = cb;
    }
}

pub fn start() {
    unsafe {
        DEVICES = MTDeviceCreateList();
        let device_num = CFArrayGetCount(DEVICES);
        for i in 0..device_num {
            let device = CFArrayGetValueAtIndex(DEVICES, i);
            MTRegisterContactFrameCallback(device, contact_frame_callback);
            MTDeviceStart(device, 0);
        }
    }
}

pub fn stop() {
    unsafe {
        let device_num = CFArrayGetCount(DEVICES);
        for i in 0..device_num {
            let device = CFArrayGetValueAtIndex(DEVICES, i);
            MTDeviceStop(device, 0);
            MTUnregisterContactFrameCallback(device, contact_frame_callback);
        }
        CFRelease(DEVICES);
        DEVICES = ptr::null();
    }
}
