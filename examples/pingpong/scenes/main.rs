use crate::scripts::ai::AIBehaviour;
use crate::scripts::ball::BallBehaviour;
use crate::scripts::fail::FailBehaviour;
use crate::scripts::player::PlayerBehaviour;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use flame::game_object::physics::PhysicsObject;
use flame::game_object::{GameObject, GameObjectBuilder};
use flame::physics::pixels_to_physics_units;
use flame::physics::{FlameCollider, FlameColliderType};
use flame::FlameEngine;
use rapier2d::geometry::ColliderBuilder;
use rapier2d::prelude::{vector, Ball, RigidBodyBuilder};
use raylib::color::Color;

pub fn register_main_scene(mut flame: &mut FlameEngine) {
    let scene = flame.register_scene("Main".to_string());

    scene.register_ui("examples/pingpong/ui/scene1.html".to_string());

    scene
        .data_map
        .insert("ScoreValue".to_string(), "0".to_string());

    // let mut ball = GameObject::new(0,0,"Main".to_string());

    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![5.0, 2.0])
        .linear_damping(0.0)
        .angular_damping(0.0)
        .ccd_enabled(true)
        .can_sleep(false)
        .build();

    let ball_collider = FlameCollider {
        collider_type: FlameColliderType::Circle(20),
        sensor: false,
        restitution: 1.05,
        friction: 0.0,
    };

    let mut ball = GameObjectBuilder::new()
        .behaviour(BallBehaviour { speed: 1.0 })
        .rigid_body(ball_rigid_body, scene)
        .collider(ball_collider, scene)
        .graphics(GraphicsType::Circle(CircleData {
            radius: 20.0,
            color: Color::WHITE,
        }))
        .build();

    let ball_c_handle = ball.physics.collider_handle.unwrap();
    let ball_r_handle = ball.physics.rigid_body_handle.unwrap();

    scene.register_game_object(ball);

    let mut player = GameObject::new(0, 0, "Main".to_string());

    player.insert_behaviour(PlayerBehaviour {
        speed: 0.8,
        decay: 50.5,
        ball_handle: ball_c_handle,
        score: 0,
    });

    let player_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.5, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    player.attach_rigid_body(player_rigid_body, scene);

    let player_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((10, 155)), // Extra 5PX comfort zone to make it feel more fair
        sensor: false,
        restitution: 1.0,
        friction: 100.0,
    };

    let player_collider_handle = player.attach_collider_with_rigid_body(player_collider, scene);

    player.add_graphics(GraphicsType::Square(SquareData {
        width: 30,
        height: 150,
        color: Color::WHITE,
    }));
    scene.register_game_object(player);

    let mut ai = GameObject::new(0, 0, "Main".to_string());

    ai.insert_behaviour(AIBehaviour {
        speed: 1.0,
        ball_handle: ball_c_handle,
        ball_rigid_handle: ball_r_handle,
        rng: rand::thread_rng(),
    });

    let ai_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![10.0, 2.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .build();

    ai.attach_rigid_body(ai_rigid_body, scene);

    let ai_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((10, 150)),
        sensor: false,
        restitution: 1.05,
        friction: 100.0,
    };

    let ai_collider_handle = ai.attach_collider_with_rigid_body(ai_collider, scene);

    ai.add_graphics(GraphicsType::Square(SquareData {
        width: 30,
        height: 150,
        color: Color::WHITE,
    }));

    scene.register_game_object(ai);

    let mut top_wall = GameObject::new(0, 0, "Main".to_string());

    let top_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    top_wall.attach_rigid_body(top_wall_rigid_body, scene);

    let top_wall_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((500, 1)),
        sensor: false,
        restitution: 1.05,
        friction: 100.0,
    };

    let top_wall_collider_handle =
        top_wall.attach_collider_with_rigid_body(top_wall_collider, scene);

    scene.register_game_object(top_wall);

    let mut bottom_wall = GameObject::new(0, 0, "Main".to_string());

    let bottom_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 10.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    bottom_wall.attach_rigid_body(bottom_wall_rigid_body, scene);

    let bottom_wall_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((500, 1)),
        sensor: false,
        restitution: 1.05,
        friction: 100.0,
    };

    let bottom_wall_collider_handle =
        bottom_wall.attach_collider_with_rigid_body(bottom_wall_collider, scene);

    let mut fail_barrier = GameObject::new(0, 0, "Main".to_string());

    fail_barrier.insert_behaviour(FailBehaviour {
        speed: 0.0,
        ball_handle: ball_c_handle,
    });

    let fail_barrier_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    fail_barrier.attach_rigid_body(fail_barrier_rigid_body, scene);

    let fail_barrier_collider = FlameCollider {
        collider_type: FlameColliderType::Rectangle((1, 500)),
        sensor: true,
        restitution: 1.0,
        friction: 100.0,
    };

    let fail_barrier_collider_handle =
        fail_barrier.attach_collider_with_rigid_body(fail_barrier_collider, scene);

    scene.register_game_object(fail_barrier);
}
