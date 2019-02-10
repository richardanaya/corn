use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn console_log(start: *mut c_char, len: usize);
    fn console_time(start: *mut c_char, len: usize);
    fn console_timeEnd(start: *mut c_char, len: usize);
    fn global_createEventListener() -> i32;
    fn global_getWindow() -> i32;
    fn Window_requestAnimationFrame(win: i32, callback: i32);
    fn EventTarget_addEventListener(
        win: i32,
        type_start: *mut c_char,
        type_len: usize,
        callback: i32,
    );
    fn KeyboardEvent_get_keyCode(ev: i32) -> i32;
}

pub fn log(msg: &str) {
    let s = CString::new(msg).unwrap();
    let l = msg.len();
    unsafe {
        console_log(s.into_raw(), l);
    }
}

pub fn start_time(msg: &str) {
    let s = CString::new(msg).unwrap();
    let l = msg.len();
    unsafe {
        console_time(s.into_raw(), l);
    }
}

pub fn end_time(msg: &str) {
    let s = CString::new(msg).unwrap();
    let l = msg.len();
    unsafe {
        console_timeEnd(s.into_raw(), l);
    }
}

pub fn get_window() -> i32 {
    unsafe { global_getWindow() }
}

pub fn create_event_listener() -> i32 {
    unsafe { global_createEventListener() }
}

pub fn request_animation_frame(win: i32, callback: i32) {
    unsafe {
        Window_requestAnimationFrame(win, callback);
    }
}

pub fn add_event_listener(target: i32, event_name: &str, callback: i32) {
    let s = CString::new(event_name).unwrap();
    let l = event_name.len();
    unsafe {
        EventTarget_addEventListener(target, s.into_raw(), l, callback);
    }
}

pub fn keyboard_event_get_key_code(ev: i32) -> i32 {
    unsafe { KeyboardEvent_get_keyCode(ev) }
}
