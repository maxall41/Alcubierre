use crate::renderer::buffer::QuadBufferBuilder;
use flume::{Receiver, Sender};
use hashbrown::{HashMap, HashSet};
use rapier2d::geometry::{ColliderSet, Ray};
use rapier2d::math::{Point, Real, Vector};
use rapier2d::pipeline::QueryFilter;
use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RayIntersection,
    RigidBodySet,
};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use crate::EngineEvent;
use crate::game_object::{GameObject, GameObjectView};
use crate::physics::screen_units_to_physics_units;
use crate::scene::Scene;

pub trait UserBehaviour: UserBehaviourClone {
    fn game_loop(
        &mut self,
        game_object_view: GameObjectView,
        engine_view: EngineView,
        frame_delta: f32,
    );
    fn unloaded(&mut self, _engine_view: EngineView, _game_object_view: GameObjectView) {} // {} Is Optional
    fn loaded(&mut self, _engine_view: EngineView, _game_object_view: GameObjectView) {} // {} Is Optional
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

pub struct EngineView<'a> {
    pub rigid_body_set: &'a mut RigidBodySet,
    pub narrow_phase: &'a mut NarrowPhase,
    pub collider_set: &'a mut ColliderSet,
    pub(crate) event_tx: &'a mut Sender<EngineEvent>,
    pub(crate) key_locks: &'a mut HashSet<VirtualKeyCode>,
    pub(crate) keys_pressed: &'a mut HashSet<VirtualKeyCode>,
    pub(crate) query_pipeline: &'a mut QueryPipeline,
}

impl<'a> EngineView<'a> {
    pub fn is_colliding_with_sensor(&self, col1: ColliderHandle, col2: ColliderHandle) -> bool {
        if self.narrow_phase.intersection_pair(col1, col2) == Some(true) {
            true
        } else {
            false
        }
    }
    pub fn is_colliding(&self, col1: ColliderHandle, col2: ColliderHandle) -> bool {
        if let Some(contact_pair) = self.narrow_phase.contact_pair(col1, col2) {
            if contact_pair.has_any_active_contact {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn load_scene(&self, scene_name: String) {
        self.event_tx
            .send(EngineEvent::SwitchToScene(scene_name))
            .unwrap();
    }
    pub fn insert_into_datamap(&self, var: String, val: String) {
        self.event_tx
            .send(EngineEvent::InsertDatamapValue((var, val)))
            .unwrap();
    }
    pub fn set_datamap_value(&self, var: String, val: String) {
        self.event_tx
            .send(EngineEvent::SetDatamapValue((var, val)))
            .unwrap();
    }
    pub fn remove_datamap_value(&self, var: String) {
        self.event_tx
            .send(EngineEvent::RemoveDatamapValue(var))
            .unwrap();
    }
    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
    pub fn is_key_up(&self, key: VirtualKeyCode) -> bool {
        !self.keys_pressed.contains(&key)
    }
    pub fn is_key_pressed(&mut self, key: VirtualKeyCode) -> bool {
        let contains = self.keys_pressed.contains(&key);
        if contains {
            if self.key_locks.contains(&key) {
                false
            } else {
                self.key_locks.insert(key);
                true
            }
        } else {
            self.key_locks.remove(&key);
            false
        }
    }
    pub fn cast_ray(
        &mut self,
        direction: Vector<Real>,
        origin: &[f32],
        length: Real,
    ) -> Option<(RayIntersection, ColliderHandle, Ray)> {
        let x = screen_units_to_physics_units(origin[0]);
        let y = screen_units_to_physics_units(origin[1]);
        let ray = Ray::new(Point::from([x,y]), direction);

        let filter = QueryFilter::default();

        if let Some((handle, intersection)) = self.query_pipeline.cast_ray_and_get_normal(
            &self.rigid_body_set,
            &self.collider_set,
            &ray,
            length,
            true,
            filter,
        ) {
            Some((intersection, handle, ray))
        } else {
            None
        }
    }
    pub fn cast_ray_with_excluded_collider(
        &mut self,
        direction: Vector<Real>,
        origin: &[f32],
        length: Real,
        excluded_collider: ColliderHandle,
    ) -> Option<(RayIntersection, ColliderHandle, Ray)> {
        let x = screen_units_to_physics_units(origin[0]);
        let y = screen_units_to_physics_units(origin[1]);
        let ray = Ray::new(Point::from([x,y]), direction);

        let filter = QueryFilter::default().exclude_collider(excluded_collider);

        if let Some((handle, intersection)) = self.query_pipeline.cast_ray_and_get_normal(
            &self.rigid_body_set,
            &self.collider_set,
            &ray,
            length,
            true,
            filter,
        ) {
            Some((intersection, handle, ray))
        } else {
            None
        }
    }
}
