use rapier2d::prelude::{vector, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP};
use flame::FlameEngineView;
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::{is_key_down, is_key_pressed};

#[derive(Clone)]
pub struct BallBehaviour {
    pub(crate) speed: f32,
}

impl UserBehaviour for BallBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32) {

    }

    fn loaded(&mut self,engine_view: FlameEngineView,game_object_view: GameObjectView) {
        let rigid_body = engine_view.rigid_body_set.get_mut(game_object_view.physics.rigid_body_handle.unwrap()).unwrap();
        rigid_body.set_linvel(Vector::new(-13.0,0.0),true);
    }

}