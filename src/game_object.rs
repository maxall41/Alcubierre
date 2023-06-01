use crate::game_object::behaviours::UserBehaviour;
use crate::game_object::graphics::{Graphics, GraphicsType};
use crate::game_object::physics::{PhysicsData, PhysicsObject};
use crate::physics::AlcubierreCollider;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::{EngineView, EngineEvent, Scene};
use flume::Sender;
use hashbrown::HashSet;
use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::NarrowPhase;
use rapier2d::prelude::RigidBodySet;
use winit::event::VirtualKeyCode;

pub mod behaviours;
pub mod graphics;
pub mod physics;

#[derive(Clone)]
pub struct GameObject {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Box<dyn UserBehaviour + 'static>>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub physics: PhysicsData,
}

pub struct GameObjectView<'a> {
    pub physics: &'a mut PhysicsData,
    pub pos_x: &'a mut f32,
    pub pos_y: &'a mut f32,
}

impl GameObject {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        GameObject {
            graphics: None,
            behaviours: vec![],
            pos_y,
            pos_x,
            physics: PhysicsData {
                collider_handle: None,
                rigid_body_handle: None,
            },
        }
    }
    pub fn unloading(
        &mut self,
        narrow_phase: &mut NarrowPhase,
        rigid_body_set: &mut RigidBodySet,
        tx: &mut Sender<EngineEvent>,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
    ) {
        for behaviour in &mut self.behaviours {
            behaviour.unloaded(
                EngineView {
                    rigid_body_set,
                    narrow_phase,
                    event_tx: tx,
                    keys_pressed,
                    key_locks,
                },
                GameObjectView {
                    physics: &mut self.physics,
                    pos_x: &mut self.pos_x,
                    pos_y: &mut self.pos_y,
                },
            );
        }
    }
    pub fn loading(
        &mut self,
        narrow_phase: &mut NarrowPhase,
        rigid_body_set: &mut RigidBodySet,
        tx: &mut Sender<EngineEvent>,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
    ) {
        for behaviour in &mut self.behaviours {
            behaviour.loaded(
                EngineView {
                    rigid_body_set,
                    narrow_phase,
                    event_tx: tx,
                    key_locks,
                    keys_pressed,
                },
                GameObjectView {
                    physics: &mut self.physics,
                    pos_x: &mut self.pos_x,
                    pos_y: &mut self.pos_y,
                },
            );
        }
    }
    pub fn execute(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        narrow_phase: &mut NarrowPhase,
        event_tx: &mut Sender<EngineEvent>,
        buffer: &mut QuadBufferBuilder,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
    ) {
        if self.physics.rigid_body_handle.is_some() {
            let new_pos = self.get_updated_physics_position(rigid_body_set);
            self.pos_x = new_pos.0;
            self.pos_y = new_pos.1;
        }

        for behaviour in &mut self.behaviours {
            behaviour.game_loop(
                GameObjectView {
                    physics: &mut self.physics,
                    pos_x: &mut self.pos_x,
                    pos_y: &mut self.pos_y,
                },
                EngineView {
                    rigid_body_set,
                    narrow_phase,
                    event_tx,
                    keys_pressed,
                    key_locks,
                },
                0.0, //TODO
            );
        }
        self.render(buffer);
    }
}

pub struct GameObjectBuilder {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Box<dyn UserBehaviour + 'static>>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub physics: PhysicsData,
}

impl GameObjectBuilder {
    pub fn new() -> GameObjectBuilder {
        GameObjectBuilder {
            graphics: None,
            behaviours: vec![],
            pos_y: 0.0,
            pos_x: 0.0,
            physics: PhysicsData {
                collider_handle: None,
                rigid_body_handle: None,
            },
        }
    }
    pub fn graphics(mut self, graphics: GraphicsType) -> GameObjectBuilder {
        self.graphics = Some(graphics);
        self
    }
    pub fn rigid_body(mut self, rigid_body: RigidBody, scene: &mut Scene) -> GameObjectBuilder {
        let handle = scene.rigid_body_set.insert(rigid_body);
        self.physics.rigid_body_handle = Some(handle.clone());
        self
    }
    pub fn behaviour(mut self, behaviour: impl UserBehaviour) -> GameObjectBuilder {
        self.behaviours.push(Box::new(behaviour));
        self
    }
    pub fn collider(mut self, collider: AlcubierreCollider, scene: &mut Scene) -> GameObjectBuilder {
        if self.physics.rigid_body_handle.is_some() {
            let handle = scene.collider_set.insert_with_parent(
                collider.to_rapier(),
                self.physics.rigid_body_handle.unwrap(),
                &mut scene.rigid_body_set,
            );
            self.physics.collider_handle = Some(handle.clone());
        } else {
            let handle = scene.collider_set.insert(collider.to_rapier());
            self.physics.collider_handle = Some(handle.clone());
        }
        self
    }
    pub fn build(self) -> GameObject {
        GameObject {
            graphics: self.graphics,
            behaviours: self.behaviours,
            pos_x: self.pos_x,
            pos_y: self.pos_y,
            physics: self.physics,
        }
    }
}
