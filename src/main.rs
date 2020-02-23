#[macro_use]

extern crate lazy_static;
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::os::raw::{c_int, c_uint, c_long, c_float, c_double, c_void};
use std::ptr;

// CoreFoundation

type CFIndex = c_long;
type CFArrayRef = *const c_void;
type CFMutableArrayRef = *const c_void;
type CFTypeRef = *const c_void;

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;

    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;

    fn CFRelease(cf: CFTypeRef);
}

// CoreGraphics

const K_CG_EVENT_MOUSE_MOVED: c_uint = 5; // kCGEventMouseMoved

const K_CG_HID_EVENT_TAP: c_uint = 0; // kCGHIDEventTap

type CGEventRef = *const c_void;
type CGEventSourceRef = *const c_void;
type CGEventType = c_uint;
type CGMouseButton = c_uint;
type CGEventTapLocation = c_uint;
type CGFloat = c_double;

#[repr(C)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;

    fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouseType: CGEventType,
        mouseCursorPosition: CGPoint,
        mouseButton: CGMouseButton
    ) -> CGEventRef;

    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;

    fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
}

// MultitouchSupport

type MTDeviceRef = *const c_void;

#[repr(C)]
struct mtPoint {
    x: c_double,
    y: c_double,
}

#[repr(C)]
struct mtReadout {
    position: mtPoint,
    velocity: mtPoint,
}

#[repr(C)]
struct Finger {
    frame: c_int,
    timestamp: c_double,
    identifier: c_int,
    state: c_int,
    unknown1: c_int,
    unknown2: c_int,
    normalized: mtReadout,
    size: c_float,
    unknown3: c_int,
    angle: c_float,
    major_axis: c_float,
    minor_axis: c_float,
    unknown4: mtReadout,
    unknown5_1: c_int,
    unknown5_2: c_int,
    unknown6: c_float,
}

type MTContactCallbackFunction = extern fn(
    MTDeviceRef,
    *mut Finger,
    c_int,
    c_double,
    c_int
) -> c_int;

#[link(name = "MultitouchSupport", kind = "framework")]
extern {
    fn MTDeviceCreateList() -> CFMutableArrayRef;

    fn MTRegisterContactFrameCallback(
        device: MTDeviceRef,
        callback: MTContactCallbackFunction
    );

    fn MTUnregisterContactFrameCallback(
        device: MTDeviceRef,
        callback: MTContactCallbackFunction
    );

    fn MTDeviceStart(device: MTDeviceRef, _: c_int);

    fn MTDeviceStop(device: MTDeviceRef, _: c_int);
}

// cursor position

fn get_cursor_position() -> (f64, f64) {
    unsafe {
        let event = CGEventCreate(ptr::null());
        let pos = CGEventGetLocation(event);
        CFRelease(event);
        (pos.x, pos.y)
    }
}

fn set_cursor_position(x: f64, y: f64) {
    unsafe {
        let pos = CGPoint{x: x, y: y};
        let event = CGEventCreateMouseEvent(
            ptr::null(),
            K_CG_EVENT_MOUSE_MOVED,
            pos,
            0
        );
        CGEventPost(K_CG_HID_EVENT_TAP, event);
        CFRelease(event);
    }
}

// devices

fn create_devices() -> CFMutableArrayRef {
    unsafe {
        MTDeviceCreateList()
    }
}

fn release_devices(devices: CFMutableArrayRef) {
    unsafe {
        CFRelease(devices);
    }
}

fn start_devices(
    devices: CFMutableArrayRef,
    callback: MTContactCallbackFunction
) {
    unsafe {
        let device_num = CFArrayGetCount(devices);
        for i in 0..device_num {
            let device = CFArrayGetValueAtIndex(devices, i);
            MTRegisterContactFrameCallback(device, callback);
            MTDeviceStart(device, 0);
        }
    }
}

fn stop_devices(
    devices: CFMutableArrayRef,
    callback: MTContactCallbackFunction
) {
    unsafe {
        let device_num = CFArrayGetCount(devices);
        for i in 0..device_num {
            let device = CFArrayGetValueAtIndex(devices, i);
            MTDeviceStop(device, 0);
            MTUnregisterContactFrameCallback(device, callback);
        }
    }
}

// app

lazy_static! {
    static ref APP: Mutex<App> = Mutex::new(App {
        pos1: None,
        pos2: None,
        vel: None,
        last_tick: Instant::now(),
    });
}

#[derive(Debug)]
struct App {
    pos1: Option<Position>,
    pos2: Option<Position>,
    vel: Option<Velocity>,
    last_tick: Instant,
}

impl App {
    fn touch(&mut self, t: f64) {
        let (x, y) = get_cursor_position();
        if let Some(pos2) = self.pos2 {
            if pos2.x == x && pos2.y == y {
                return;
            }
        }
        self.pos1 = self.pos2;
        self.pos2 = Some(Position { x, y, t });
        self.vel = None;
    }

    fn release(&mut self) {
        if let (Some(pos1), Some(pos2)) = (self.pos1, self.pos2) {
            let dx = pos2.x - pos1.x;
            let dy = pos2.y - pos1.y;
            let dt = pos2.t - pos1.t;
            self.vel = Some(Velocity { x: dx / dt, y: dy / dt });
            self.pos1 = None;
            self.pos2 = None;
        }
    }

    fn tick(&mut self) {
        let prev_tick = self.last_tick;
        self.last_tick = Instant::now();

        if let Some(mut vel) = self.vel {
            // カーソル移動
            let (x, y) = get_cursor_position();
            let dt = (self.last_tick - prev_tick).as_secs_f64();
            set_cursor_position(x + vel.x * dt, y + vel.y * dt);

            // 減速
            vel.decelerate(5.0 * dt);
            if vel.length() < 30.0 {
                self.vel = None;
            } else {
                self.vel = Some(vel);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
    t: f64,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: f64,
    y: f64,
}

impl Velocity {
    fn decelerate(&mut self, r: f64) {
        self.x *= 1.0 - r;
        self.y *= 1.0 - r;
    }

    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// callback

extern fn callback(
    _device: MTDeviceRef,
    data: *mut Finger,
    data_num: c_int,
    timestamp: c_double,
    _frame: c_int
) -> c_int {
    let touched = unsafe {
        (0..data_num).any(|i| (*data.offset(i as isize)).state == 4)
    };
    if touched {
        APP.lock().unwrap().touch(timestamp);
    } else {
        APP.lock().unwrap().release();
    }
    0
}

// main

fn main() {
    // SIGINT, SIGTERM で終了
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // デバイスの開始処理
    let devices = create_devices();
    start_devices(devices, callback);

    // メインループ
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(10));
        APP.lock().unwrap().tick();
    }

    // デバイスの終了処理
    stop_devices(devices, callback);
    release_devices(devices);
}
