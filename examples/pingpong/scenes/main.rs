use crate::scripts::ai::AIBehaviour;
use crate::scripts::ball::BallBehaviour;
use crate::scripts::fail::FailBehaviour;
use crate::scripts::player::PlayerBehaviour;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::{GameObject, GameObjectBuilder};
use alcubierre::physics::screen_units_to_physics_units;
use alcubierre::physics::{AlcubierreCollider, AlcubierreColliderType};
use alcubierre::Engine;
use rapier2d::geometry::ColliderBuilder;
use rapier2d::prelude::{vector, Ball, RigidBodyBuilder};
use alcubierre::ui::frontend::RGBColor;

pub fn register_main_scene(mut flame: &mut Engine) {
    let scene = flame.register_scene("Main".to_string());

    scene.register_ui("examples/pingpong/ui/scene1.html".to_string());

    scene
        .data_map
        .insert("ScoreValue".to_string(), "0".to_string());

    let mut ball = GameObject::new(0.0,0.0);

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

    let mut ball = GameObjectBuilder::new()
        .behaviour(BallBehaviour { speed: 0.001 })
        .rigid_body(ball_rigid_body, scene)
        .collider(ball_collider, scene)
        .graphics(GraphicsType::Circle(CircleData {
            radius: 0.6,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255
            },
        }))
        .build();

    let ball_c_handle = ball.physics.collider_handle.unwrap();
    let ball_r_handle = ball.physics.rigid_body_handle.unwrap();

    scene.register_game_object(ball);

    let mut player = GameObject::new(0.0, 0.0);

    player.insert_behaviour(PlayerBehaviour {
        speed: 0.01,
        decay: 50.5,
        ball_handle: ball_c_handle,
        score: 0,
    });

    let player_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![-0.1, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();

    player.attach_rigid_body(player_rigid_body, scene);

    let player_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((0.5, 6.0)), // Extra 5PX comfort zone to make it feel more fair
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let player_collider_handle = player.attach_collider_with_rigid_body(player_collider, scene);

    player.add_graphics(GraphicsType::Rect(RectData {
        width: 0.5,
        height: 6.0,
        color: RGBColor {
            red: 255,
            green: 255,
            blue: 255
        },
    }));

    scene.register_game_object(player);

    let mut ai = GameObject::new(0.0, 0.0);

    ai.insert_behaviour(AIBehaviour {
        speed: 1.0,
        ball_handle: ball_c_handle,
        ball_rigid_handle: ball_r_handle,
        rng: rand::thread_rng(),
    });

    let ai_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.1, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .build();

    ai.attach_rigid_body(ai_rigid_body, scene);

    let ai_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Rectangle((0.5, 6.0)),
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let ai_collider_handle = ai.attach_collider_with_rigid_body(ai_collider, scene);

    ai.add_graphics(GraphicsType::Rect(RectData {
        width: 0.5,
        height: 6.0,
        color: RGBColor {
            red: 255,
            green: 255,
            blue: 255
        },
    }));

    scene.register_game_object(ai);
    //
    // let mut top_wall = GameObject::new(0.0, 0.0);
    //
    // let top_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
    //     .translation(vector![0.0, 0.0])
    //     .linear_damping(5.5)
    //     .angular_damping(2.0)
    //     .can_sleep(false)
    //     .ccd_enabled(true)
    //     .build();
    //
    // top_wall.attach_rigid_body(top_wall_rigid_body, scene);
    //
    // let top_wall_collider = AlcubierreCollider {
    //     collider_type: AlcubierreColliderType::Rectangle((30.0, 0.5)),
    //     sensor: false,
    //     restitution: 1.05,
    //     friction: 100.0,
    // };
    //
    // let top_wall_collider_handle =
    //     top_wall.attach_collider_with_rigid_body(top_wall_collider, scene);
    //
    // scene.register_game_object(top_wall);
    //
    // let mut bottom_wall = GameObject::new(0.0, 0.0);
    //
    // let bottom_wall_rigid_body = RigidBodyBuilder::kinematic_position_based()
    //     .translation(vector![0.0, 10.0])
    //     .linear_damping(5.5)
    //     .angular_damping(2.0)
    //     .can_sleep(false)
    //     .ccd_enabled(true)
    //     .build();
    //
    // bottom_wall.attach_rigid_body(bottom_wall_rigid_body, scene);
    //
    // let bottom_wall_collider = AlcubierreCollider {
    //     collider_type: AlcubierreColliderType::Rectangle((30.0, 0.5)),
    //     sensor: false,
    //     restitution: 1.05,
    //     friction: 100.0,
    // };
    //
    // let bottom_wall_collider_handle =
    //     bottom_wall.attach_collider_with_rigid_body(bottom_wall_collider, scene);
    //
    // scene.register_game_object(bottom_wall);
    //
    // let mut fail_barrier = GameObject::new(0.0, 0.0);
    //
    // fail_barrier.insert_behaviour(FailBehaviour {
    //     speed: 0.0,
    //     ball_handle: ball_c_handle,
    // });
    //
    // let fail_barrier_rigid_body = RigidBodyBuilder::kinematic_position_based()
    //     .translation(vector![0.0, 0.0])
    //     .linear_damping(5.5)
    //     .angular_damping(2.0)
    //     .can_sleep(false)
    //     .ccd_enabled(true)
    //     .build();
    //
    // fail_barrier.attach_rigid_body(fail_barrier_rigid_body, scene);
    //
    // let fail_barrier_collider = AlcubierreCollider {
    //     collider_type: AlcubierreColliderType::Rectangle((0.5, 30.0)),
    //     sensor: true,
    //     restitution: 1.0,
    //     friction: 100.0,
    // };
    //
    // let fail_barrier_collider_handle =
    //     fail_barrier.attach_collider_with_rigid_body(fail_barrier_collider, scene);
    //
    // scene.register_game_object(fail_barrier);
}
