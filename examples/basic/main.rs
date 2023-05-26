use lazy_static::lazy_static;
use raylib::color::Color;
use flame::FlameEngine;
use flame::game_object::GameObject;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType};
use std::sync::RwLock;
use std::sync::Arc;
use rapier2d::geometry::{Collider, ColliderBuilder};
use flame::game_object::physics::PhysicsObject;

mod player;

use crate::player::PlayerBehaviour;


fn game_code(engine: &mut FlameEngine) {
}

#[tokio::main]
async fn main() {
    let mut flame = FlameEngine::new();

    let mut player = GameObject::new(50,50);

    player.insert_behaviour(PlayerBehaviour { speed: 500.0 });

    let player_collider = ColliderBuilder::cuboid(50.0, 50.0).build();

    player.attach_collider(player_collider,&mut flame);

    player.add_graphics(GraphicsType::Circle(CircleData {
        radius: 20.0,
        color: Color::RED,
    }));
    flame.insert_game_object(player);

    flame.start_cycle(game_code);
    println!("Cycle started");
}