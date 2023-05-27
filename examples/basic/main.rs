use lazy_static::lazy_static;
use raylib::color::Color;
use flame::FlameEngine;
use flame::game_object::GameObject;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use std::sync::RwLock;
use std::sync::Arc;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{RigidBodyBuilder, vector};
use flame::game_object::physics::PhysicsObject;
use crate::scenes::main::register_main_scene;
use crate::scenes::second::register_second_scene;
use crate::scripts::gateway::GatewayBehaviour;

mod scripts;
mod scenes;

use crate::scripts::player::PlayerBehaviour;


fn game_code(engine: &mut FlameEngine) {
}

#[tokio::main]
async fn main() {
    let mut flame = FlameEngine::new();

    register_main_scene(&mut flame);

    register_second_scene(&mut flame);

    flame.set_current_scene("Main".to_string());

    flame.start_cycle(game_code);
    println!("Cycle started");
}