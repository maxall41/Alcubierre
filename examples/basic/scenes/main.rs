use std::f32::consts::PI;
use crate::scripts::player::PlayerBehaviour;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::GameObject;

use alcubierre::physics::{AlcubierreCollider, AlcubierreColliderType};
use alcubierre::ui::frontend::RGBColor;
use alcubierre::Engine;

use rapier2d::prelude::{vector, RigidBodyBuilder};

pub fn register_main_scene(flame: &mut Engine) {
    let scene = flame.register_scene("Main".to_string());

    let mut player = GameObject::new(0.0, 0.0);

    player.insert_behaviour(PlayerBehaviour { speed: 0.045 });

    let player_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .build();

    player.attach_rigid_body(player_rigid_body, scene);

    let player_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Circle(0.4),
        sensor: false,
        restitution: 0.0,
        friction: 0.1,
    };

    let _player_collider_handle = player.attach_collider_with_rigid_body(player_collider, scene);

    player.add_graphics(GraphicsType::Circle(CircleData {
        radius: 0.4,
        color: RGBColor {
            red: 255,
            green: 255,
            blue: 255,
        },
    }));
    scene.register_game_object(player);

    let mut ground = GameObject::new(0.0, 0.0);

    let ground_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, -0.1])
        .build();

    ground.attach_rigid_body(ground_rigid_body, scene);

    let ground_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((10.0, 0.8)),
        sensor: false,
        restitution: 0.0,
        friction: 0.0,
    };

    ground.attach_collider_with_rigid_body(ground_collider, scene);

    ground.add_graphics(GraphicsType::Rect(RectData {
        color: RGBColor {
            red: 235,
            green: 64,
            blue: 52,
        },
        width: 10.0,
        height: 0.8,
    }));

    scene.register_game_object(ground);
    //
    // let mut gateway = GameObject::new(0.0, 0.0);
    //
    // let gateway_rigid_body = RigidBodyBuilder::fixed()
    //     .translation(vector![
    //         screen_units_to_physics_units(100),
    //         screen_units_to_physics_units(370)
    //     ])
    //     .build();
    //
    // gateway.attach_rigid_body(gateway_rigid_body, scene);
    //
    // let gateway_collider = FlameCollider {
    //     collider_type: FlameColliderType::Rectangle((50, 80)),
    //     sensor: true,
    //     restitution: 0.7,
    //     friction: 0.0,
    // };
    //
    // gateway.attach_collider_with_rigid_body(gateway_collider, scene);
    //
    // gateway.insert_behaviour(GatewayBehaviour {
    //     player_collider: player_collider_handle,
    //     going_to_next: false,
    //     scene_to_switch_to: "Second".to_string(),
    // });
    //
    // gateway.add_graphics(GraphicsType::Square(SquareData {
    //     color: RGBColor {
    //         red: 52,
    //         green: 235,
    //         blue: 55,
    //     },
    //     width: 50.0,
    //     height: 80.0,
    // }));
    //
    // scene.register_game_object(gateway);
}
