use flume::Sender;
use rapier2d::geometry::NarrowPhase;
use rapier2d::prelude::{ColliderSet, RigidBodyHandle, RigidBodySet};
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::GetFrameTime;
use crate::{FlameEngine, FlameEngineView, FlameEvent};
use crate::game_object::behaviours::{UserBehaviour};
use crate::game_object::graphics::{Graphics, GraphicsType};
use crate::game_object::physics::{PhysicsData, PhysicsObject};

pub mod physics;
pub mod graphics;
pub mod behaviours;


pub struct GameObject {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Box<dyn UserBehaviour + 'static>>,
    pub pos_x: i32,
    pub pos_y: i32,
    pub physics: PhysicsData
}

pub struct GameObjectView<'a> {
    pub physics: &'a mut PhysicsData,
    pub pos_x: &'a mut i32,
    pub pos_y: &'a mut i32
}

impl GameObject {
    pub fn new(pos_x: i32,pos_y: i32) -> Self {
        GameObject {
            graphics: None,
            behaviours: vec![],
            pos_y,
            pos_x,
            physics: PhysicsData {
                collider_handle: None,
                rigid_body_handle: None
            }
        }
    }
    pub fn unloading(&mut self,rigid_body_set: &mut RigidBodySet,narrow_phase: &mut NarrowPhase,event_tx: &mut Sender<FlameEvent>) {
        for behaviour in &mut self.behaviours {
            behaviour.scene_unloaded(GameObjectView {
                physics: &mut self.physics,
                pos_x: &mut self.pos_x,
                pos_y: &mut self.pos_y,
            }, FlameEngineView {
                rigid_body_set,
                narrow_phase,
                event_tx
            });
        }
    }
    pub fn loading(&mut self,rigid_body_set: &mut RigidBodySet,narrow_phase: &mut NarrowPhase,event_tx: &mut Sender<FlameEvent>) {
        for behaviour in &mut self.behaviours {
            behaviour.scene_loaded(GameObjectView {
                physics: &mut self.physics,
                pos_x: &mut self.pos_x,
                pos_y: &mut self.pos_y,
            }, FlameEngineView {
                rigid_body_set,
                narrow_phase,
                event_tx
            });
        }
    }
    pub fn execute(&mut self,d: &mut RaylibDrawHandle,rigid_body_set: &mut RigidBodySet,narrow_phase: &mut NarrowPhase,event_tx: &mut Sender<FlameEvent>) {

        let mut time: f32 = 0.0;
        unsafe {
            time = GetFrameTime();
        }

        if self.physics.rigid_body_handle.is_some() {
            let new_pos = self.get_updated_physics_position(rigid_body_set);
            self.pos_x = new_pos.0 as i32;
            self.pos_y = new_pos.1 as i32;
        }

        for behaviour in &mut self.behaviours {
            behaviour.game_loop(GameObjectView {
                physics: &mut self.physics,
                pos_x: &mut self.pos_x,
                pos_y: &mut self.pos_y,
            }, FlameEngineView {
                rigid_body_set,
                narrow_phase,
                event_tx
            }, time);
        }
        self.render(d);
    }

}
