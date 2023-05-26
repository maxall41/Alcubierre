use lazy_static::lazy_static;
use raylib::color::Color;
use flame::FlameEngine;
use flame::game_object::GameObject;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType};
use std::sync::RwLock;
use std::sync::Arc;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{RigidBodyBuilder, vector};
use flame::game_object::physics::PhysicsObject;

mod player;

use crate::player::PlayerBehaviour;


fn game_code(engine: &mut FlameEngine) {
}

#[tokio::main]
async fn main() {
    let mut flame = FlameEngine::new();

    let mut player = GameObject::new(50,50);

    player.insert_behaviour(PlayerBehaviour { speed: 60.0 });

    let player_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .build();

    player.attach_rigid_body(player_rigid_body,&mut flame);

    let player_collider = ColliderBuilder::ball(20.0).restitution(0.7).build();

    player.attach_collider_with_rigid_body(player_collider,&mut flame);

    player.add_graphics(GraphicsType::Circle(CircleData {
        radius: 20.0,
        color: Color::RED,
    }));
    flame.insert_game_object(player);

    flame.start_cycle(game_code);
    println!("Cycle started");
}