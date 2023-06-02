use alcubierre::audio::basic::AudioSourceBuilder;
use alcubierre::game_object::behaviours::EngineView;
use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::GameObjectView;
use rapier2d::prelude::{vector, Vector};
use winit::event::VirtualKeyCode;

#[derive(Clone)]
pub struct PlayerBehaviour {
    pub(crate) speed: f32,
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, mut engine_view: EngineView) {
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
            let d = engine_view.cast_ray_with_excluded_collider(
                vector![0.0, -0.3],
                &[
                    game_object_view.pos_x.clone(),
                    game_object_view.pos_y.clone(),
                ],
                0.3,
                game_object_view.physics.collider_handle.unwrap(),
            );
            if d.is_none() {
                return;
            }
            let (i, _h, _ray) = d.unwrap();

            // println!("{}",i.toi);

            // If we are on the ground jump
            if i.toi < 0.1 {
                let file = AudioSourceBuilder::new()
                    .path("examples/basic/Jump34.wav")
                    .build();
                engine_view.play_sound(file);
                println!("Added Y_vel!");
                y_vel += 1.3;
            }
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
