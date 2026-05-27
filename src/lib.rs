#[macro_use]
mod browser;
mod engine;
mod sound;
mod segment;
mod obstacle;
mod platform;
mod enemy;
mod player;
mod game;
mod beam;
mod barrier;
mod create_platform;
mod create_enemy;
mod enemy_rocket;
mod enemy_okayu;
mod enemy_beam;
mod enemy_energy;
mod special;
mod special_kazama;
mod special_okanyan;
mod bonus_balloon;
mod bonus;
mod bonus_wood_box;
mod data_for_playing;
mod enemy_fubura;
mod enemy_kedama;
mod enemy_hato;
mod enemy_pokobe;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::engine::GameLoop;
use crate::game::FlyingOkanyan;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    console_error_panic_hook::set_once();

    browser::spawn_local(async move {

        // ここにループ処理を書く
        let game = FlyingOkanyan::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });

    Ok(())
}
