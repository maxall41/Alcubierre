pub mod audio;
mod events;
pub mod game_object;
pub mod physics;
mod renderer;
pub mod scene;
pub mod ui;

use crate::renderer::buffer::QuadBufferBuilder;
use flume::{Receiver, Sender};
use hashbrown::{HashMap, HashSet};
use kira::manager::backend::DefaultBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundData;
use macroquad::color::{BLUE, DARKGRAY, GREEN, RED, YELLOW};
use macroquad::prelude::{clear_background, draw_circle, draw_line, draw_rectangle, draw_text, next_frame, screen_height, screen_width};
use nalgebra::SMatrix;
use rapier2d::geometry::ColliderSet;
use std::ops::Add;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::events::EngineEvent;
use crate::renderer::Render;
use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager,
    MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RigidBodySet,
};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::scene::Scene;

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
    physics_pipeline: PhysicsPipeline,
    gravity: SMatrix<f32, 2, 1>,
    audio_manager: AudioManager,
    renderer: Option<Render>,
}

pub struct EngineConfig {
    pub gravity: f32,
}

impl Engine {
    pub fn new(window_width: i32, window_height: i32, config: EngineConfig) -> Self {
        let (event_tx, event_rx) = flume::bounded(60); //TODO: Set to frame rate

        let query_pipeline = QueryPipeline::new();

        let mut physics_pipeline = PhysicsPipeline::new();

        let gravity = vector![0.0, config.gravity];

        let mut audio_manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

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
            audio_manager,
            physics_pipeline,
            gravity,
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

    pub fn start_cycle(mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();




        let mut next_redraw: Instant = Instant::now();

        let till_next = Duration::from_millis(1000 / fps);

        self.renderer = Some(futures::executor::block_on(renderer::Render::new(
            &window,
            current_size,
        )));

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
                    self.renderer.as_mut().unwrap().resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &mut so w have to dereference it twice
                    self.renderer.as_mut().unwrap().resize(**new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                self.draw();
            }
            Event::MainEventsCleared => {
                let active_scene_unwrapped = self.active_scene.as_mut().unwrap();
                self.physics_pipeline.step(
                    &self.gravity,
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

                let current = Instant::now();
                if current >= next_redraw {
                    self.draw();
                }
                next_redraw = Instant::now() + till_next;
            }
            _ => *control_flow = ControlFlow::WaitUntil(Instant::now().add(till_next)),
        });
    }
    fn draw(&mut self) {
        let active_scene = self.active_scene.as_mut();
        let mut buffer = QuadBufferBuilder::new();

        if active_scene.is_some() {
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

        self.renderer.as_mut().unwrap().render_buffer(buffer);
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
