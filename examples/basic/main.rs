use crate::scenes::main::register_main_scene;
use crate::scenes::second::register_second_scene;
use crate::scripts::gateway::GatewayBehaviour;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use flame::game_object::physics::PhysicsObject;
use flame::game_object::GameObject;
use flame::FlameEngine;
use lazy_static::lazy_static;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{vector, RigidBodyBuilder};
use raylib::color::Color;
use std::sync::Arc;
use std::sync::RwLock;

mod scenes;
mod scripts;

use crate::scripts::player::PlayerBehaviour;

fn game_code(engine: &mut FlameEngine) {}

#[tokio::main]
async fn main() {
    let mut flame = FlameEngine::new(640, 480);

    register_main_scene(&mut flame);

    register_second_scene(&mut flame);

    flame.set_current_scene("Main".to_string());

    flame.start_cycle(game_code, FlameConfig { gravity: 65.24 });
    println!("Cycle started");
}
