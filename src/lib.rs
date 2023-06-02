pub mod game_object;
pub mod physics;
pub mod ui;
pub mod scene;
mod events;
pub mod audio;

use std::ops::Add;
use std::thread::sleep;
use std::time::{Duration, Instant};
use flume::{Receiver, Sender};
use hashbrown::{HashMap, HashSet};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::StaticSoundData;
use macroquad::color::{BLUE, DARKGRAY, GREEN, RED, YELLOW};
use macroquad::prelude::{clear_background, draw_circle, draw_line, draw_rectangle, draw_text, next_frame, screen_height, screen_width};
use nalgebra::SMatrix;
use rapier2d::geometry::{ColliderSet};


use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline,
    RigidBodySet,
};
use crate::events::EngineEvent;

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
    gravity: SMatrix<f32,2,1>,
    audio_manager: AudioManager,
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

        let mut audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

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

        // macroquad::Window::new("Engine", self.m_primary());




    }
    // async fn m_primary() {
    //     loop {
    //         clear_background(RED);
    //
    //         draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //         draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //         draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    //
    //         draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
    //
    //         next_frame().await
    //     }
    // }
    fn draw(&mut self,) {
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
