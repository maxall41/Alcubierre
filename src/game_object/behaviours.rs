use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;
use crate::{FlameEngine, FlameEngineView};
use crate::game_object::{GameObject, GameObjectView};
use crate::keyboard::{is_key_down, is_key_pressed, is_key_released, is_key_up};

pub trait UserBehaviour: 'static {
    fn game_loop(&mut self,game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32);
    fn init(&mut self);
    fn scene_unloaded(&mut self) {} // {} Is Optional
    fn scene_loaded(&mut self) {} // {} Is Optional
}


impl GameObject {
    pub fn insert_behaviour(&mut self, mut behaviour: impl UserBehaviour) {
        behaviour.init();
        self.behaviours.push(Box::new(behaviour));
    }
}