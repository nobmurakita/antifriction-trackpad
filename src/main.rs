use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use antifriction_trackpad::app::App;
use antifriction_trackpad::trackpad;

lazy_static::lazy_static! {
    static ref APP: Mutex<App> = Mutex::new(App::default());
}

fn callback(touched: bool, timestamp: f64) {
    if touched {
        APP.lock().unwrap().touch(timestamp);
    } else {
        APP.lock().unwrap().release();
    }
}

fn main() {
    // SIGINT, SIGTERM で終了
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // トラックパッド開始
    trackpad::set_callback(callback);
    trackpad::start();

    // メインループ
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(10));
        APP.lock().unwrap().tick();
    }

    // トラックパッド停止
    trackpad::stop();
}
