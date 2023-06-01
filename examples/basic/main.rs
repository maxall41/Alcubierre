use crate::scenes::main::register_main_scene;
use crate::scripts::gateway::GatewayBehaviour;
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
    let mut engine = Engine::new(640, 480);

    register_main_scene(&mut engine);

    // register_second_scene(&mut flame);

    engine.set_current_scene("Main".to_string());

    engine.start_cycle(EngineConfig { gravity: -0.8 });
    println!("Cycle started");
}
