pub mod audio;
mod events;
pub mod game_object;
pub mod physics;
mod renderer;
pub mod scene;
pub mod ui;

use crate::renderer::buffer::QuadBufferBuilder;
use hashbrown::{HashMap, HashSet};
use instant::Instant;
use kanal::{Receiver, Sender};
use kira::manager::backend::DefaultBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use log::warn;
use nalgebra::{SMatrix, Vector2};
use rapier2d::geometry::ColliderSet;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use ui::frontend::RGBColor;

use crate::events::EngineEvent;
use crate::game_object::behaviours::EngineView;
use crate::renderer::Render;
use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RigidBodySet,
};
use winit::dpi::PhysicalSize;
use winit::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::scene::Scene;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct MouseData {
    is_middle_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    mouse_position: Vector2<f64>,
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
    collision_locks: HashSet<ColliderHandle>,
    mouse_data: MouseData,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
    config: EngineConfig,
    audio_manager: AudioManager,
    renderer: Option<Render>,
    last_delta: Duration,
    last_frame_end: Instant,
}

pub struct EngineConfig {
    pub gravity: f32,
    pub clear_color: RGBColor,
}

impl Engine {
    pub fn new(window_width: i32, window_height: i32, config: EngineConfig) -> Self {
        let (event_tx, event_rx) = kanal::bounded(60); //TODO: Set to frame rate

        let query_pipeline = QueryPipeline::new();

        let mut physics_pipeline = PhysicsPipeline::new();

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
            collision_locks: HashSet::new(),
            query_pipeline,
            audio_manager,
            physics_pipeline,
            config,
            renderer: None,
            mouse_data: MouseData {
                is_left_pressed: false,
                is_right_pressed: false,
                is_middle_pressed: false,
                mouse_position: Vector2::new(0.0, 0.0),
            },
            last_delta: Duration::from_millis(0),
            last_frame_end: Instant::now(),
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
                    &mut self.last_delta,
                    &mut self.collision_locks,
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
                &mut self.last_delta,
                &mut self.collision_locks,
            )
        }
    }

    pub fn start_cycle(mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(940, 640));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    canvas.set_id("game-canvas");
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let current_size = PhysicalSize::new(1080, 940);

        window.set_inner_size(current_size);

        self.renderer = Some(pollster::block_on(renderer::Render::new(
            &window,
            current_size,
        )));

        let dt_physics = 1.0 / 60.0;
        let mut accumulator = 0.0;

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
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => {
                    self.mouse_data.is_left_pressed = *state == ElementState::Pressed;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Right,
                    state,
                    ..
                } => {
                    self.mouse_data.is_right_pressed = *state == ElementState::Pressed;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_data.mouse_position = Vector2::new(position.x, position.y);
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Middle,
                    state,
                    ..
                } => {
                    self.mouse_data.is_middle_pressed = *state == ElementState::Pressed;
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {}
            Event::MainEventsCleared => {
                // Cap FPS at 60FPS. With practically no minimum
                // Only sleep on native. When running in WASM browser controls FPS via requestAnimationFrame.
                cfg_if::cfg_if! {
                    if #[cfg(target_arch = "wasm32")] {

                    } else {
                        sleep(Duration::from_millis(16));
                    }
                }

                let active_scene_unwrapped = self.active_scene.as_mut().unwrap();

                while accumulator >= dt_physics {
                    self.physics_pipeline.step(
                        &vector![0.0, self.config.gravity],
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

                    accumulator -= dt_physics;
                }

                self.draw();

                self.last_delta = Instant::now() - self.last_frame_end;

                accumulator += self.last_delta.as_millis() as f32 / 1000.0;

                self.last_frame_end = Instant::now();

                //
            }
            _ => *control_flow = ControlFlow::Poll,
        });
    }
    fn draw(&mut self) {
        let active_scene = self.active_scene.as_mut();
        let mut buffer = QuadBufferBuilder::new();

        if active_scene.is_some() {
            {
                let active_scene = self.active_scene.as_mut().unwrap();

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
                        &mut self.last_delta,
                        &mut self.collision_locks,
                    );
                }
            }

            self.handle_events();

            let active_scene = self.active_scene.as_mut().unwrap();

            self.renderer.as_mut().unwrap().render_buffer(
                buffer,
                &active_scene.ui_ast,
                &active_scene.data_map,
                &active_scene.function_map,
                &mut EngineView {
                    rigid_body_set: &mut active_scene.rigid_body_set,
                    narrow_phase: &mut active_scene.narrow_phase_collision,
                    collider_set: &mut active_scene.collider_set,
                    event_tx: &mut self.event_tx,
                    key_locks: &mut self.key_locks,
                    keys_pressed: &mut self.keys_pressed,
                    query_pipeline: &mut self.query_pipeline,
                    frame_delta: &mut self.last_delta,
                    collision_locks: &mut self.collision_locks,
                },
                &self.mouse_data,
                &self.config.clear_color,
            );
        }
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
                current_game_object_id: 0,
            },
        );
        self.scenes.get_mut(&scene_name).unwrap()
    }
}
