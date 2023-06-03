use crate::scenes::fail::register_fail_scene;
use crate::scenes::main::register_main_scene;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::GameObject;
use alcubierre::{Engine, EngineConfig};
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{vector, RigidBodyBuilder};
use std::sync::Arc;
use std::sync::RwLock;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

mod scenes;
mod scripts;

use crate::scripts::player::PlayerBehaviour;


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut flame = Engine::new(
        640,
        480,
        EngineConfig {
            gravity: 0.0,
            // clear_color: Color::BLACK,
        },
    );

    register_main_scene(&mut flame);

    register_fail_scene(&mut flame);

    flame.set_current_scene("Main".to_string());

    flame.start_cycle();
    println!("Cycle started");

    Ok(())
}
