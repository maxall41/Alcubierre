use alcubierre::game_object::behaviours::{EngineView, UserBehaviour};
use alcubierre::game_object::{GameObject, GameObjectView};
use rapier2d::geometry::ColliderHandle;
use rapier2d::math::Isometry;
use rapier2d::prelude::{vector, Vector};
use winit::event::VirtualKeyCode;

#[derive(Clone)]
pub struct PlayerBehaviour {
    pub(crate) speed: f32,
    pub(crate) decay: f32,
    pub(crate) ball_handle: ColliderHandle,
    pub(crate) score: i32,
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, mut engine_view: EngineView) {
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

        if engine_view
            .is_colliding_once(
                game_object_view.physics.collider_handle.unwrap(),
                self.ball_handle,
            )
            .is_some()
        {
            self.score += 1;
            engine_view.set_datamap_value("ScoreValue".to_string(), self.score.to_string());
        }
    }
}
