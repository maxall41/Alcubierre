use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType, RectData};
use alcubierre::game_object::physics::PhysicsObject;
use alcubierre::game_object::{GameObject, GameObjectBuilder};
use alcubierre::physics::screen_units_to_physics_units;
use alcubierre::physics::{AlcubierreCollider, AlcubierreColliderType};
use alcubierre::ui::frontend::RGBColor;
use alcubierre::Engine;
use rand::thread_rng;
use rapier2d::geometry::ColliderBuilder;
use rapier2d::math::Isometry;
use rapier2d::prelude::{vector, Ball, RigidBodyBuilder};
use winit::event::VirtualKeyCode;
use alcubierre::game_object::graphics::SpriteData;

use alcubierre::game_object::behaviours::EngineView;
use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::GameObjectView;
use rapier2d::prelude::Vector;

#[derive(Clone)]
pub struct BallBehaviour {
    pub(crate) speed: f32,
}

impl UserBehaviour for BallBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, engine_view: EngineView) {
        let mut y_vel: f32 = 0.0;
        if engine_view.is_key_down(VirtualKeyCode::Up) {
            y_vel += self.speed * engine_view.frame_delta.as_millis() as f32;
        }
        if engine_view.is_key_down(VirtualKeyCode::Down) {
            y_vel -= self.speed * engine_view.frame_delta.as_millis() as f32;
        }

        let rigid_body = engine_view
            .rigid_body_set
            .get_mut(game_object_view.physics.rigid_body_handle.unwrap())
            .unwrap();


        let pos = rigid_body.position();
        rigid_body.set_position(
            Isometry::new(vector![pos.translation.x, pos.translation.y + y_vel], 0.0),
            true,
        );
    }

    fn loaded(&mut self, engine_view: EngineView, game_object_view: GameObjectView) {}
}

pub fn register_main_scene(mut flame: &mut Engine) {

    let scene = flame.register_scene("Main".to_string());

    scene.register_ui(include_str!("ui/test.html"));

    scene
        .data_map
        .insert("ScoreValue".to_string(), "0".to_string());

    let ball_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 0.0])
        .linear_damping(5.5)
        .angular_damping(2.0)
        .can_sleep(false)
        .build();

    let ball_collider = AlcubierreCollider {
        collider_type: AlcubierreColliderType::Circle(0.6),
        sensor: false,
        restitution: 1.0,
        friction: 0.0,
    };

    let mut ball_builder = GameObjectBuilder::new()
        .behaviour("ball.rhai")
        .rigid_body(ball_rigid_body)
        .collider(ball_collider)
        .graphics(GraphicsType::Sprite(SpriteData {
            width: 1.0,
            height: 1.0,
            sprite_id: "tile000".to_string(),
            flip_h: false,
            flip_v: false
        }));

    let ball = scene.register_game_object(ball_builder);

    let ball_c_handle = ball.physics.collider_handle.unwrap();
    let ball_r_handle = ball.physics.rigid_body_handle.unwrap();
}
