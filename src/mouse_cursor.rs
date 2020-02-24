use crate::framework::core_foundation::*;
use crate::framework::core_graphics::*;
use std::ptr;

pub fn get_position() -> (f64, f64) {
    unsafe {
        let event = CGEventCreate(ptr::null());
        let pos = CGEventGetLocation(event);
        CFRelease(event);
        (pos.x, pos.y)
    }
}

pub fn set_position(x: f64, y: f64) {
    unsafe {
        let event = CGEventCreateMouseEvent(
            ptr::null(),
            kCGEventMouseMoved,
            CGPoint {x: x, y: y},
            0
        );
        CGEventPost(kCGHIDEventTap, event);
        CFRelease(event);
    }
}
