pub mod game_object;
pub mod keyboard;
pub mod helpers;

use std::thread::sleep;
use std::time::Duration;
use flume::{Receiver, Sender};
use hashbrown::HashMap;
use rapier2d::geometry::{ColliderBuilder, ColliderSet};
use rapier2d::prelude::{BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodySet, vector};
use raylib::{RaylibHandle, RaylibThread};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::ffi::{GetKeyPressed, IsKeyPressed};
use crate::game_object::GameObject;
use crate::game_object::graphics::Graphics;

pub struct FlameEngineView<'a> {
    pub rigid_body_set: &'a mut RigidBodySet,
    narrow_phase: &'a mut NarrowPhase,
    event_tx:&'a mut Sender<FlameEvent>
}

impl<'a> FlameEngineView<'a> {
    pub fn is_colliding_with_sensor(&self, col1: ColliderHandle, col2: ColliderHandle) -> bool {
        if self.narrow_phase.intersection_pair(col1, col2) == Some(true) {
           true
        } else {
            false
        }
    }
    pub fn load_scene(&self, scene_name: String) {
        self.event_tx.send(FlameEvent::SwitchToScene(scene_name)).unwrap();
    }
}

pub struct Scene {
    pub game_objects: Vec<GameObject>,
}

pub enum FlameEvent {
    SwitchToScene(String)
}

pub struct FlameEngine {
    raylib: RaylibHandle,
    thread: RaylibThread,
    collider_set: ColliderSet,
    pub rigid_body_set: RigidBodySet,
    pub scenes: HashMap<String,Scene>,
    current_scene: Option<String>,
    narrow_phase_collision: NarrowPhase,
    event_rx: Receiver<FlameEvent>,
    event_tx: Sender<FlameEvent>
}


impl FlameEngine {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("Hello, World")
            .build();

        let collider_set = ColliderSet::new();

        let rigid_body_set = RigidBodySet::new();

        let (event_tx, event_rx) = flume::bounded(60); //TODO: Set to frame rate


        FlameEngine {
            raylib: rl,
            thread: thread,
            scenes: HashMap::new(),
            collider_set,
            rigid_body_set,
            current_scene: None,
            narrow_phase_collision: NarrowPhase::new(),
            event_tx,
            event_rx
        }
    }

    fn set_current_scene(&mut self,new_scene: String) {
        let scene = self.scenes.get_mut(self.current_scene.as_ref().unwrap()).unwrap();

        for object in &mut scene.game_objects {
            object.unloading()
        }

        self.current_scene = Some(new_scene);

        let new_scene = self.scenes.get_mut(self.current_scene.as_ref().unwrap()).unwrap();

        for object in &mut new_scene.game_objects {
            object.loading()
        }
    }

    pub fn start_cycle(&mut self,game_code: fn(&mut Self)) {


        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, 65.24]; // We should scale this instead
        // 850.81
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();


        loop {
            if self.current_scene.as_ref().is_some() {
                physics_pipeline.step(
                    &gravity,
                    &integration_parameters,
                    &mut island_manager,
                    &mut broad_phase,
                    &mut self.narrow_phase_collision,
                    &mut self.rigid_body_set,
                    &mut self.collider_set,
                    &mut impulse_joint_set,
                    &mut multibody_joint_set,
                    &mut ccd_solver,
                    None,
                    &physics_hooks,
                    &event_handler,
                );

               let packet =  self.event_rx.try_recv();
                match packet {
                    Ok(event) => {
                        match event {
                            FlameEvent::SwitchToScene(scene) => {
                                self.current_scene = Some(scene);
                            },
                        }
                    },
                    Err(e) => {

                    }
                }

                { game_code(self); }

                let mut d = self.raylib.begin_drawing(&self.thread);

                let scene = self.scenes.get_mut(self.current_scene.as_ref().unwrap()).unwrap();

                for object in &mut scene.game_objects {
                    object.execute(&mut d,&mut self.rigid_body_set,&mut self.narrow_phase_collision,&mut self.event_tx);
                }

                d.clear_background(Color::WHITE);

            }

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    pub fn insert_game_object(&mut self,game_object: GameObject,scene_name: String) {
        let scene = self.scenes.get_mut(&scene_name);
        match scene {
            Some(scene) => {
                scene.game_objects.push(game_object);
            },
            None => {
                self.scenes.insert(scene_name,Scene {
                    game_objects: vec![game_object]
                });
            }
        };
    }
}
