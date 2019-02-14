use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn console_log(start: *mut c_char);
    fn console_time(start: *mut c_char);
    fn console_timeEnd(start: *mut c_char);
    fn Document_querySelector(doc: i32, start: *mut c_char) -> i32;
    fn global_createEventListener() -> i32;
    fn global_getWindow() -> i32;
    fn Window_requestAnimationFrame(win: i32, callback: i32);
    fn Window_get_document(win: i32) -> i32;
    fn EventTarget_addEventListener(
        win: i32,
        type_start: *mut c_char,
        callback: i32,
    );
    fn KeyboardEvent_get_keyCode(ev: i32) -> i32;
    fn HTMLCanvasElement_getContext(ctx: i32, start: *mut c_char) -> i32;
}

pub fn log(msg: &str) {
    unsafe {
        console_log(CString::new(msg).unwrap().into_raw());
    }
}

pub fn start_time(msg: &str) {
    unsafe {
        console_time(CString::new(msg).unwrap().into_raw());
    }
}

pub fn end_time(msg: &str) {
    unsafe {
        console_timeEnd(CString::new(msg).unwrap().into_raw());
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
    unsafe {
        EventTarget_addEventListener(target,  CString::new(event_name).unwrap().into_raw(), callback);
    }
}

pub fn keyboard_event_get_key_code(ev: i32) -> i32 {
    unsafe { KeyboardEvent_get_keyCode(ev) }
}

pub fn get_document(win: i32) -> i32 {
    unsafe { Window_get_document(win) }
}

pub fn query_selector(doc: i32, query: &str) -> i32 {
    unsafe { Document_querySelector(doc, CString::new(query).unwrap().into_raw()) }
}

pub fn get_context(canvas: i32, ctx: &str) -> i32 {
    unsafe { HTMLCanvasElement_getContext(canvas, CString::new(ctx).unwrap().into_raw()) }
}
