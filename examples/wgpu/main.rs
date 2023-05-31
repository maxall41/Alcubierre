mod player;

use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use flame::game_object::physics::PhysicsObject;
use flame::game_object::GameObject;
use flame::{FlameConfig, FlameEngine};
use lazy_static::lazy_static;
use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::{vector, RigidBodyBuilder};
use std::sync::Arc;
use std::sync::RwLock;
use flame::physics::{FlameCollider, FlameColliderType};
use flame::ui::frontend::RGBColor;
use crate::player::PlayerBehaviour;

fn game_code(engine: &mut FlameEngine) {}

#[tokio::main]
async fn main() {
    let mut flame = FlameEngine::new(640, 480);


    let scene = flame.register_scene("Main".to_string());

    let mut player = GameObject::new(0, 0);

    player.insert_behaviour(PlayerBehaviour { speed: 1.0 });

    let player_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .build();

    player.attach_rigid_body(player_rigid_body, scene);

    let player_collider = FlameCollider {
        collider_type: FlameColliderType::Circle(20),
        sensor: false,
        restitution: 0.7,
        friction: 0.0,
    };

    let player_collider_handle = player.attach_collider_with_rigid_body(player_collider, scene);

    player.add_graphics(GraphicsType::Circle(CircleData {
        radius: 20.0,
        color: RGBColor {
            red: 0,
            green: 0,
            blue: 0
        },
    }));
    scene.register_game_object(player);


    flame.set_current_scene("Main".to_string());

    flame.start_cycle(
        game_code,
        FlameConfig {
            gravity: 0.0,
            // clear_color: Color::BLACK,
        },
    );
    println!("Cycle started");
}
