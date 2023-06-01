use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::{GameObject, GameObjectView};
use alcubierre::EngineView;
use rapier2d::prelude::{vector, Vector};
use winit::event::VirtualKeyCode;

#[derive(Clone)]
pub struct PlayerBehaviour {
    pub(crate) speed: f32,
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(
        &mut self,
        game_object_view: GameObjectView,
        mut engine_view: EngineView,
        frame_delta: f32,
    ) {
        let mut x_vel: f32 = 0.0;
        let mut y_vel: f32 = 0.0;
        if engine_view.is_key_down(VirtualKeyCode::Right) {
            // *view.pos_x += (self.speed * frame_delta) as i32;
            x_vel += self.speed;
        }
        if engine_view.is_key_down(VirtualKeyCode::Left) {
            // *view.pos_x  -= (self.speed * frame_delta) as i32;
            x_vel -= self.speed;
        }
        if engine_view.is_key_pressed(VirtualKeyCode::Space) {
            // *view.pos_y -= (self.speed * frame_delta) as i32;
            y_vel += 0.8;
        }

        let rigid_body = engine_view
            .rigid_body_set
            .get_mut(game_object_view.physics.rigid_body_handle.unwrap())
            .unwrap();

        let current_vel = rigid_body.linvel();

        rigid_body.set_linvel(
            Vector::new(current_vel.x + x_vel, current_vel.y + y_vel),
            true,
        );
    }
}
