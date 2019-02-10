mod browser;
mod game;
use crate::browser::*;
use crate::game::*;
use ref_thread_local::RefThreadLocal;
#[macro_use]
extern crate ref_thread_local;

ref_thread_local! {
    static managed WINDOW: i32 = get_window();
    static managed ANIMATION_CALLBACK: i32 = create_event_listener();
    static managed KEYDOWN_CALLBACK: i32 = create_event_listener();
    static managed KEYUP_CALLBACK: i32 = create_event_listener();
    static managed GAME_STATE: GameState = GameState::new();
}

#[no_mangle]
pub fn callback(callback_handle: i32, e: i32) {
    let animation_callback = *ANIMATION_CALLBACK.borrow();
    let keydown_callback = *KEYDOWN_CALLBACK.borrow();
    let keyup_callback = *KEYUP_CALLBACK.borrow();
    let game_state = &mut *GAME_STATE.borrow_mut();
    if callback_handle == animation_callback {
        run(game_state);
        request_animation_frame(*WINDOW.borrow(), animation_callback);
        return;
    } else if callback_handle == keydown_callback {
        let k = keyboard_event_get_key_code(e);
        key_down(game_state,k);
        return;
    }
    else if callback_handle == keyup_callback {
        let k = keyboard_event_get_key_code(e);
        key_up(game_state,k);
        return;
    }
    log(&format!("unhandled callback {:?}",callback_handle))
}

#[no_mangle]
pub fn main() -> () {
    request_animation_frame(*WINDOW.borrow(), *ANIMATION_CALLBACK.borrow());
    add_event_listener(*WINDOW.borrow(), "keydown", *KEYDOWN_CALLBACK.borrow());
    add_event_listener(*WINDOW.borrow(), "keyup", *KEYUP_CALLBACK.borrow());
}
