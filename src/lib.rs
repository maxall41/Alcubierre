pub mod game_object;
pub mod physics;
mod renderer;
pub mod ui;
pub mod scene;
mod events;

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
use crate::physics::screen_units_to_physics_units;
use crate::scene::Scene;

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

        let query_pipeline = QueryPipeline::new();

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

        let current_size = PhysicalSize::new(1080, 940);

        window.set_inner_size(current_size);

        let mut render = pollster::block_on(renderer::Render::new(&window, current_size));

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
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
                    let active_scene_unwrapped = active_scene.unwrap();
                    physics_pipeline.step(
                        &gravity,
                        &active_scene_unwrapped.integration_params,
                        &mut active_scene_unwrapped.island_manager,
                        &mut active_scene_unwrapped.broad_phase,
                        &mut active_scene_unwrapped.narrow_phase_collision,
                        &mut active_scene_unwrapped.rigid_body_set,
                        &mut active_scene_unwrapped.collider_set,
                        &mut active_scene_unwrapped.impulse_joint_set,
                        &mut active_scene_unwrapped.multibody_joint_set,
                        &mut active_scene_unwrapped.ccd_solver,
                        None,
                        &(),
                        &(),
                    );

                    self.query_pipeline.update(
                        &active_scene_unwrapped.rigid_body_set,
                        &active_scene_unwrapped.collider_set,
                    );

                    self.handle_events();

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
