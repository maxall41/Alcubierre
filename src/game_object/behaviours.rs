use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;

use crate::game_object::{GameObject, GameObjectView};
use crate::{FlameEngine, FlameEngineView};

pub trait UserBehaviour: UserBehaviourClone {
    fn game_loop(
        &mut self,
        game_object_view: GameObjectView,
        engine_view: FlameEngineView,
        frame_delta: f32,
    );
    fn unloaded(&mut self, engine_view: FlameEngineView, game_object_view: GameObjectView) {} // {} Is Optional
    fn loaded(&mut self, engine_view: FlameEngineView, game_object_view: GameObjectView) {} // {} Is Optional
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
    pub fn insert_behaviour(&mut self, behaviour: impl UserBehaviour) {
        self.behaviours.push(Box::new(behaviour));
    }
}
