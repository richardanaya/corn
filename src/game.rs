use crate::rendering::*;
const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const SPRITE_WIDTH: i32 = 32;
const SPRITE_HEIGHT: i32 = 32;
const SPRITES_PER_ROW:i32  = 6;

pub struct GameState {
    sprites: i32,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState { sprites: -1 };
    }
}

pub fn init(game_state: &mut GameState) {
    game_state.sprites = load_image("corn");
}

pub fn run(game_state: &mut GameState) {
    clear(0, 0, WIDTH, HEIGHT);
    for x in 0..(WIDTH/SPRITE_WIDTH) {
        for y in 0..(HEIGHT/SPRITE_HEIGHT) {
            draw_sprite(game_state,1,x*SPRITE_WIDTH,y*SPRITE_HEIGHT);
        }
    }
}

fn draw_sprite(game_state: &mut GameState, i:i32,x:i32,y:i32){
    draw_image(game_state.sprites, (i%SPRITES_PER_ROW)*SPRITE_WIDTH, (i/SPRITES_PER_ROW)*SPRITE_HEIGHT, SPRITE_WIDTH, SPRITE_HEIGHT, x, y, SPRITE_WIDTH, SPRITE_HEIGHT);
}

pub fn key_down(_game_state: &mut GameState, _key_code: i32) {}

pub fn key_up(_game_state: &mut GameState, _key_code: i32) {}
