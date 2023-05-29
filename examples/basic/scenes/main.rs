use crate::scripts::gateway::GatewayBehaviour;
use crate::scripts::player::PlayerBehaviour;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use flame::game_object::physics::PhysicsObject;
use flame::game_object::GameObject;
use flame::physics::pixels_to_physics_units;
use flame::physics::{FlameCollider, FlameColliderType};
use flame::FlameEngine;
use rapier2d::geometry::ColliderBuilder;
use rapier2d::prelude::{vector, RigidBodyBuilder};
use raylib::color::Color;

pub fn register_main_scene(flame: &mut FlameEngine) {
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
        color: Color::RED,
    }));
    scene.register_game_object(player);

    let mut ground = GameObject::new(0, 0);

    let ground_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, pixels_to_physics_units(450)])
        .build();

    ground.attach_rigid_body(ground_rigid_body, scene);

    let ground_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((640, 30)),
        sensor: false,
        restitution: 0.7,
        friction: 0.0,
    };

    ground.attach_collider_with_rigid_body(ground_collider, scene);

    ground.add_graphics(GraphicsType::Square(SquareData {
        color: Color::BLUE,
        width: 640,
        height: 30,
    }));

    scene.register_game_object(ground);

    let mut gateway = GameObject::new(0, 0);

    let gateway_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            pixels_to_physics_units(100),
            pixels_to_physics_units(370)
        ])
        .build();

    gateway.attach_rigid_body(gateway_rigid_body, scene);

    let gateway_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((50, 80)),
        sensor: true,
        restitution: 0.7,
        friction: 0.0,
    };

    gateway.attach_collider_with_rigid_body(gateway_collider, scene);

    gateway.insert_behaviour(GatewayBehaviour {
        player_collider: player_collider_handle,
        going_to_next: false,
        scene_to_switch_to: "Second".to_string(),
    });

    gateway.add_graphics(GraphicsType::Square(SquareData {
        color: Color::GREEN,
        width: 50,
        height: 80,
    }));

    scene.register_game_object(gateway);
}
