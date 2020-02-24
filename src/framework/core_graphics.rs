#![allow(non_upper_case_globals)]

use std::os::raw::{c_uint, c_double, c_void};

pub const kCGEventMouseMoved: c_uint = 5;
pub const kCGHIDEventTap: c_uint = 0;

pub type CGEventRef = *const c_void;
pub type CGEventSourceRef = *const c_void;
pub type CGEventType = c_uint;
pub type CGMouseButton = c_uint;
pub type CGEventTapLocation = c_uint;
pub type CGFloat = c_double;

#[repr(C)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;

    pub fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouseType: CGEventType,
        mouseCursorPosition: CGPoint,
        mouseButton: CGMouseButton
    ) -> CGEventRef;

    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;

    pub fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
}
