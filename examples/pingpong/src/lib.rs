use crate::scenes::main::register_main_scene;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::GameObject;
use alcubierre::ui::frontend::RGBColor;
use alcubierre::{Engine, EngineConfig};
use log::warn;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{vector, RigidBodyBuilder};
use std::sync::Arc;
use std::sync::RwLock;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

mod scenes;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() -> Result<(), JsValue> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    warn!("Started");

    let mut flame = Engine::new(
        640,
        480,
        EngineConfig {
            gravity: 0.0,
            clear_color: RGBColor {
                red: 0,
                green: 0,
                blue: 0,
            },
        }
    );
    flame.load_sprite_atlas("atlas.json","atlas.png");

    register_main_scene(&mut flame);

    //
    flame.set_current_scene("Main".to_string());
    //
    flame.start_cycle();
    println!("Cycle started");

    Ok(())
}
