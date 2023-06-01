use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::{GameObjectView};
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
        _frame_delta: f32,
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
            println!("{},{}", game_object_view.pos_x, game_object_view.pos_y);
            let d = engine_view.cast_ray_with_excluded_collider(
                vector![0.0, -0.3],
                &[game_object_view.pos_x.clone(),game_object_view.pos_y.clone()],
                0.3,
                game_object_view.physics.collider_handle.unwrap(),
            );
            if d.is_none() {
                println!("Hit nothing!");
                return;
            }
            let (i, _h, _ray) = d.unwrap();
            println!("{}",i.toi);

            // If we are on the ground jump
            if i.toi < 0.01 {
                y_vel += 0.8;
            }
            // *view.pos_y -= (self.speed * frame_delta) as i32;
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
