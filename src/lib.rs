pub mod game_object;
pub mod physics;
mod renderer;
pub mod ui;
pub mod scene;

use crate::game_object::graphics::Graphics;
use crate::game_object::GameObject;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::ui::frontend::HyperFoilAST;
use crate::ui::parse_file;
use flume::{Receiver, Sender};
use hashbrown::{HashMap, HashSet};
use nalgebra::Vector2;
use rapier2d::geometry::{ColliderBuilder, ColliderSet, Ray};
use rapier2d::math::{Point, Real, Vector};
use rapier2d::pipeline::QueryFilter;
use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RayIntersection,
    RigidBodySet,
};
use std::thread::sleep;
use std::time::Duration;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};
use crate::physics::screen_units_to_physics_units;
use crate::scene::Scene;

pub struct EngineView<'a> {
    pub rigid_body_set: &'a mut RigidBodySet,
    pub narrow_phase: &'a mut NarrowPhase,
    pub collider_set: &'a mut ColliderSet,
    event_tx: &'a mut Sender<EngineEvent>,
    key_locks: &'a mut HashSet<VirtualKeyCode>,
    keys_pressed: &'a mut HashSet<VirtualKeyCode>,
    query_pipeline: &'a mut QueryPipeline,
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

pub enum EngineEvent {
    SwitchToScene(String),
    SetDatamapValue((String, String)),
    InsertDatamapValue((String, String)),
    RemoveDatamapValue(String),
}

pub struct Engine {
    pub scenes: HashMap<String, Scene>,
    active_scene: Option<Scene>,
    event_rx: Receiver<EngineEvent>,
    event_tx: Sender<EngineEvent>,
    window_width: i32,
    window_height: i32,
    keys_pressed: HashSet<VirtualKeyCode>,
    key_locks: HashSet<VirtualKeyCode>,
    query_pipeline: QueryPipeline,
}

pub struct EngineConfig {
    pub gravity: f32,
}

impl Engine {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let (event_tx, event_rx) = flume::bounded(60); //TODO: Set to frame rate

        let mut query_pipeline = QueryPipeline::new();

        Engine {
            scenes: HashMap::new(),
            event_tx,
            window_width,
            event_rx,
            active_scene: None,
            window_height,
            key_locks: HashSet::new(),
            keys_pressed: HashSet::new(),
            query_pipeline,
        }
    }

    pub fn set_current_scene(&mut self, new_scene: String) {
        if self.active_scene.is_some() {
            let scene = self.active_scene.as_mut().unwrap();
            for object in &mut scene.game_objects {
                object.unloading(
                    &mut scene.narrow_phase_collision,
                    &mut scene.rigid_body_set,
                    &mut self.event_tx,
                    &mut self.keys_pressed,
                    &mut self.key_locks,
                    &mut self.query_pipeline,
                    &mut scene.collider_set,
                )
            }
        }

        let new_scene = self.scenes.get(&new_scene).unwrap();

        self.active_scene = Some(new_scene.clone());

        let active_scene = self.active_scene.as_mut().unwrap();

        for object in &mut active_scene.game_objects {
            object.loading(
                &mut active_scene.narrow_phase_collision,
                &mut active_scene.rigid_body_set,
                &mut self.event_tx,
                &mut self.keys_pressed,
                &mut self.key_locks,
                &mut self.query_pipeline,
                &mut active_scene.collider_set,
            )
        }
    }

    pub fn start_cycle(mut self, config: EngineConfig) {
        let gravity = vector![0.0, config.gravity];

        let mut physics_pipeline = PhysicsPipeline::new();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut current_size = PhysicalSize::new(1080, 940);

        window.set_inner_size(current_size);

        let mut render = pollster::block_on(renderer::Render::new(&window, current_size));

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                // WindowEvent::CloseRequested
                // | WindowEvent::KeyboardInput {
                //     input:
                //     KeyboardInput {
                //         state: ElementState::Pressed,
                //         virtual_keycode: Some(VirtualKeyCode::Escape),
                //         ..
                //     },
                //     ..
                // } => *control_flow = ControlFlow::Exit,
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: element_state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    // keys_pressed.push(*key);
                    match element_state {
                        ElementState::Released => {
                            self.keys_pressed.remove(key);
                        }
                        ElementState::Pressed => {
                            self.keys_pressed.insert(*key);
                        }
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    render.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &mut so w have to dereference it twice
                    render.resize(**new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let active_scene = self.active_scene.as_mut();
                let mut buffer = QuadBufferBuilder::new();

                if active_scene.is_some() {
                    let active_scene_unwraped = active_scene.unwrap();
                    physics_pipeline.step(
                        &gravity,
                        &active_scene_unwraped.integration_params,
                        &mut active_scene_unwraped.island_manager,
                        &mut active_scene_unwraped.broad_phase,
                        &mut active_scene_unwraped.narrow_phase_collision,
                        &mut active_scene_unwraped.rigid_body_set,
                        &mut active_scene_unwraped.collider_set,
                        &mut active_scene_unwraped.impulse_joint_set,
                        &mut active_scene_unwraped.multibody_joint_set,
                        &mut active_scene_unwraped.ccd_solver,
                        None,
                        &(),
                        &(),
                    );

                    self.query_pipeline.update(
                        &active_scene_unwraped.rigid_body_set,
                        &active_scene_unwraped.collider_set,
                    );

                    let packet = self.event_rx.try_recv();
                    match packet {
                        Ok(event) => match event {
                            EngineEvent::SwitchToScene(scene) => {
                                self.set_current_scene(scene);
                            }
                            EngineEvent::SetDatamapValue((var, val)) => {
                                *self
                                    .active_scene
                                    .as_mut()
                                    .unwrap()
                                    .data_map
                                    .get_mut(&var)
                                    .unwrap() = val;
                            }
                            EngineEvent::InsertDatamapValue((var, val)) => {
                                self.active_scene
                                    .as_mut()
                                    .unwrap()
                                    .data_map
                                    .insert(var, val);
                            }
                            EngineEvent::RemoveDatamapValue(var) => {
                                self.active_scene.as_mut().unwrap().data_map.remove(&var);
                            }
                        },
                        Err(e) => {
                            // panic!("{}",e); //TODO: Handle
                        }
                    }

                    let active_scene = self.active_scene.as_mut().unwrap();

                    // if active_scene.ui_ast.is_some() {
                    // }

                    for object in &mut active_scene.game_objects {
                        object.execute(
                            &mut active_scene.rigid_body_set,
                            &mut active_scene.narrow_phase_collision,
                            &mut self.event_tx,
                            &mut buffer,
                            &mut self.keys_pressed,
                            &mut self.key_locks,
                            &mut self.query_pipeline,
                            &mut active_scene.collider_set,
                        );
                    }

                    // d.clear_background(config.clear_color);
                }

                render.render_buffer(buffer);
                window.request_redraw();
            }
            _ => {}
        });
    }
    pub fn register_scene(&mut self, scene_name: String) -> &mut Scene {
        let integration_params = IntegrationParameters::default();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let narrow_phase = NarrowPhase::new();

        self.scenes.insert(
            scene_name.clone(),
            Scene {
                game_objects: vec![],
                rigid_body_set: RigidBodySet::new(),
                collider_set: ColliderSet::new(),
                narrow_phase_collision: narrow_phase,
                island_manager,
                broad_phase,
                impulse_joint_set,
                multibody_joint_set,
                ccd_solver,
                integration_params,
                ui_ast: None,
                function_map: HashMap::new(),
                data_map: HashMap::new(),
            },
        );
        self.scenes.get_mut(&scene_name).unwrap()
    }
}
