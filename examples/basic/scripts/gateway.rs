use rapier2d::prelude::{ColliderHandle, vector, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP};
use flame::FlameEngineView;
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::{is_key_down, is_key_pressed};

#[derive(Clone)]
pub struct GatewayBehaviour {
    pub(crate) player_collider: ColliderHandle,
    pub going_to_next: bool,
    pub scene_to_switch_to: String
}

impl UserBehaviour for GatewayBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView,mut engine_view: FlameEngineView,frame_delta: f32) {
        if engine_view.is_colliding_with_sensor(game_object_view.physics.collider_handle.unwrap(),self.player_collider) && self.going_to_next == false {
            self.going_to_next = true;
            engine_view.load_scene(self.scene_to_switch_to.clone())
        }
    }

}