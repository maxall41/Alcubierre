use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;
use crate::{FlameEngine, FlameEngineView};
use crate::game_object::{GameObject, GameObjectView};
use crate::keyboard::{is_key_down, is_key_pressed, is_key_released, is_key_up};

pub trait UserBehaviour: UserBehaviourClone {
    fn game_loop(&mut self,game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32);
    fn unloaded(&mut self,engine_view: FlameEngineView,game_object_view: GameObjectView) {} // {} Is Optional
    fn loaded(&mut self,engine_view: FlameEngineView,game_object_view: GameObjectView) {} // {} Is Optional
}

pub trait UserBehaviourClone: 'static {
    fn clone_box(&self) -> Box<dyn UserBehaviour>;
}

impl<T> UserBehaviourClone for T
    where
        T: 'static + UserBehaviour + Clone,
{
    fn clone_box(&self) -> Box<dyn UserBehaviour> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserBehaviour> {
    fn clone(&self) -> Box<dyn UserBehaviour> {
        self.clone_box()
    }
}


impl GameObject {
    pub fn insert_behaviour(&mut self, mut behaviour: impl UserBehaviour) {
        self.behaviours.push(Box::new(behaviour));
    }
}