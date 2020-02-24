use std::os::raw::{c_long, c_void};

pub type CFIndex = c_long;
pub type CFArrayRef = *const c_void;
pub type CFMutableArrayRef = *const c_void;
pub type CFTypeRef = *const c_void;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;

    pub fn CFArrayGetValueAtIndex(
      theArray: CFArrayRef,
      idx: CFIndex
    ) -> *const c_void;

    pub fn CFRelease(cf: CFTypeRef);
}
