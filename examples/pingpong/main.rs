use crate::scenes::fail::register_fail_scene;
use crate::scenes::main::register_main_scene;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::GameObject;
use alcubierre::{EngineConfig, Engine};
use lazy_static::lazy_static;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{vector, RigidBodyBuilder};
use std::sync::Arc;
use std::sync::RwLock;

mod scenes;
mod scripts;

use crate::scripts::player::PlayerBehaviour;


#[tokio::main]
async fn main() {
    let mut flame = Engine::new(640, 480,EngineConfig {
        gravity: 0.0,
        // clear_color: Color::BLACK,
    });

    register_main_scene(&mut flame);

    register_fail_scene(&mut flame);

    flame.set_current_scene("Main".to_string());

    flame.start_cycle();
    println!("Cycle started");
}
