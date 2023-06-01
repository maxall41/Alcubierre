use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::{GameObject, GameObjectView};
use alcubierre::EngineView;
use rapier2d::prelude::{vector, ColliderHandle, Vector};

#[derive(Clone)]
pub struct GatewayBehaviour {
    pub(crate) player_collider: ColliderHandle,
    pub going_to_next: bool,
    pub scene_to_switch_to: String,
}

impl UserBehaviour for GatewayBehaviour {
    fn game_loop(
        &mut self,
        game_object_view: GameObjectView,
        mut engine_view: EngineView,
        frame_delta: f32,
    ) {
        if engine_view.is_colliding_with_sensor(
            game_object_view.physics.collider_handle.unwrap(),
            self.player_collider,
        ) && self.going_to_next == false
        {
            self.going_to_next = true;
            println!("NEXT!");
            // engine_view.load_scene(self.scene_to_switch_to.clone())
        }
    }
}
