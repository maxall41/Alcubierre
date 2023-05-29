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

#[derive(Clone)]
pub struct GameObject {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Box<dyn UserBehaviour + 'static>>,
    pub pos_x: i32,
    pub pos_y: i32,
    pub physics: PhysicsData,
    scene: String
}

pub struct GameObjectView<'a> {
    pub physics: &'a mut PhysicsData,
    pub pos_x: &'a mut i32,
    pub pos_y: &'a mut i32
}

impl GameObject {
    pub fn new(pos_x: i32,pos_y: i32,scene: String) -> Self {
        GameObject {
            graphics: None,
            behaviours: vec![],
            pos_y,
            pos_x,
            physics: PhysicsData {
                collider_handle: None,
                rigid_body_handle: None
            },
            scene
        }
    }
    pub fn unloading(&mut self,narrow_phase: &mut NarrowPhase, rigid_body_set: &mut RigidBodySet, mut tx: &mut Sender<FlameEvent>) {
        for behaviour in &mut self.behaviours {
            behaviour.unloaded(FlameEngineView {
                rigid_body_set,
                narrow_phase,
                event_tx: tx,
            },GameObjectView {
                physics: &mut self.physics,
                pos_x: &mut self.pos_x,
                pos_y: &mut self.pos_y,
            });
        }
    }
    pub fn loading(&mut self, narrow_phase: &mut NarrowPhase, rigid_body_set: &mut RigidBodySet, mut tx: &mut Sender<FlameEvent>) {
        for behaviour in &mut self.behaviours {
            behaviour.loaded(FlameEngineView {
                rigid_body_set,
                narrow_phase,
                event_tx: tx,
            }, GameObjectView {
                physics: &mut self.physics,
                pos_x: &mut self.pos_x,
                pos_y: &mut self.pos_y,
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
