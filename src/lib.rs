pub mod game_object;
pub mod keyboard;

use std::thread::sleep;
use std::time::Duration;
use rapier2d::geometry::{ColliderBuilder, ColliderSet};
use rapier2d::prelude::{BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodySet, vector};
use raylib::{RaylibHandle, RaylibThread};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::ffi::{GetKeyPressed, IsKeyPressed};
use crate::game_object::GameObject;
use crate::game_object::graphics::Graphics;

pub struct FlameEngineView<'a> {
    pub rigid_body_set: &'a mut RigidBodySet
}

pub struct FlameEngine {
    raylib: RaylibHandle,
    thread: RaylibThread,
    game_objects: Vec<GameObject>,
    collider_set: ColliderSet,
    pub rigid_body_set: RigidBodySet
}


impl FlameEngine {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("Hello, World")
            .build();

        let collider_set = ColliderSet::new();

        let rigid_body_set = RigidBodySet::new();


        FlameEngine {
            raylib: rl,
            thread: thread,
            game_objects: vec![],
            collider_set,
            rigid_body_set
        }
    }
    pub fn start_cycle(&mut self,game_code: fn(&mut Self)) {


        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, 0.0]; // We should scale this instead
        // 850.81
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();


        loop {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut self.rigid_body_set,
                &mut self.collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );

            { game_code(self); }
            let mut d = self.raylib.begin_drawing(&self.thread);

            for object in &mut self.game_objects {
                object.execute(&mut d,&mut self.rigid_body_set);
            }

            d.clear_background(Color::WHITE);

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    pub fn insert_game_object(&mut self,game_object: GameObject) {
        self.game_objects.push(game_object);
    }
}
