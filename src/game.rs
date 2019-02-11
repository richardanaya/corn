use crate::rendering::*;
const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub struct GameState {
    img: i32,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState { img: -1 };
    }
}

pub fn init(game_state: &mut GameState) {
    game_state.img = load_image("sprite.png");
}

pub fn run(game_state: &mut GameState) {
    clear(0,0,WIDTH,HEIGHT);
    draw_image(game_state.img, 0, 0);
}

pub fn key_down(_game_state: &mut GameState, _key_code: i32) {}

pub fn key_up(_game_state: &mut GameState, _key_code: i32) {}
