use crate::scripts::ai::AIBehaviour;
use crate::scripts::ball::BallBehaviour;
use crate::scripts::barrier::BarrierBehaviour;
use crate::scripts::fail::FailBehaviour;
use crate::scripts::player::PlayerBehaviour;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::{GameObject, GameObjectBuilder};
use alcubierre::physics::screen_units_to_physics_units;
use alcubierre::physics::{AlcubierreCollider, AlcubierreColliderType};
use alcubierre::ui::frontend::RGBColor;
use alcubierre::Engine;
use rand::thread_rng;
use rapier2d::geometry::ColliderBuilder;
use rapier2d::prelude::{vector, Ball, RigidBodyBuilder};

pub fn register_main_scene(mut flame: &mut Engine) {
    let scene = flame.register_scene("Main".to_string());

    scene.register_ui(include_str!("ui/scene1.html"));

    scene
        .data_map
        .insert("ScoreValue".to_string(), "0".to_string());

    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 0.0])
        .linear_damping(0.0)
        .angular_damping(0.0)
        .ccd_enabled(true)
        .can_sleep(false)
        .build();

    let ball_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Circle(0.6),
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let mut ball_builder = GameObjectBuilder::new()
        .behaviour(BallBehaviour {})
        .rigid_body(ball_rigid_body)
        .collider(ball_collider)
        .graphics(GraphicsType::Circle(CircleData {
            radius: 0.6,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255,
            },
        }));

    let ball = scene.register_game_object(ball_builder);

    let ball_c_handle = ball.physics.collider_handle.unwrap();
    let ball_r_handle = ball.physics.rigid_body_handle.unwrap();

    let player_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![-0.1, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    let player_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((0.5, 6.0)), // Extra 5PX comfort zone to make it feel more fair
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let mut player_builder = GameObjectBuilder::new()
        .behaviour(PlayerBehaviour {
            speed: 0.0008,
            decay: 50.5,
            ball_handle: ball_c_handle,
            score: 0,
        })
        .rigid_body(player_rigid_body)
        .collider(player_collider)
        .graphics(GraphicsType::Rect(RectData {
            width: 0.5,
            height: 6.0,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255,
            },
        }));

    let player = scene.register_game_object(player_builder);

    let ai_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.1, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .build();

    let ai_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((0.5, 6.0)),
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let mut ai_builder = GameObjectBuilder::new()
        .behaviour(AIBehaviour {
            speed: 1.0,
            ball_handle: ball_c_handle,
            ball_rigid_handle: ball_r_handle,
            rng: rand::thread_rng(),
        })
        .rigid_body(ai_rigid_body)
        .collider(ai_collider)
        .graphics(GraphicsType::Rect(RectData {
            width: 0.5,
            height: 6.0,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255,
            },
        }));

    let ai = scene.register_game_object(ai_builder);

    let top_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 0.15])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    let top_wall_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((30.0, 0.5)),
        sensor: false,
        restitution: 1.05,
        friction: 0.0,
    };

    let mut top_wall_builder = GameObjectBuilder::new()
        .rigid_body(top_wall_rigid_body)
        .behaviour(BarrierBehaviour {
            ball_handle: ball_c_handle,
            ball_rigid_handle: ball_r_handle,
            rng: thread_rng(),
        })
        .collider(top_wall_collider);

    let top_wall = scene.register_game_object(top_wall_builder);

    let top_wall_collider_handle = top_wall.physics.collider_handle.unwrap();

    let bottom_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, -0.15])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    let bottom_wall_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((30.0, 0.5)),
        sensor: false,
        restitution: 1.05,
        friction: 0.0,
    };

    let mut bottom_wall_builder = GameObjectBuilder::new()
        .rigid_body(bottom_wall_rigid_body)
        .behaviour(BarrierBehaviour {
            ball_handle: ball_c_handle,
            ball_rigid_handle: ball_r_handle,
            rng: thread_rng(),
        })
        .collider(bottom_wall_collider);

    let bottom_wall = scene.register_game_object(bottom_wall_builder);

    let bottom_wall_collider_handle = bottom_wall.physics.collider_handle.unwrap();

    let fail_barrier_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![-0.2, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    let fail_barrier_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((0.5, 30.0)),
        sensor: true,
        restitution: 1.0,
        friction: 100.0,
    };

    let mut fail_barrier_builder = GameObjectBuilder::new()
        .behaviour(FailBehaviour {
            speed: 0.0,
            ball_handle: ball_c_handle,
        })
        .rigid_body(fail_barrier_rigid_body)
        .collider(fail_barrier_collider);

    let fail_barrier = scene.register_game_object(fail_barrier_builder);

    let fail_barrier_collider_handle = fail_barrier.physics.collider_handle.unwrap();
}
