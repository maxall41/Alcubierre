pub mod game_object;
pub mod keyboard;
pub mod physics;
pub mod ui;
pub mod camera;

use crate::game_object::graphics::Graphics;
use crate::game_object::GameObject;
use crate::ui::frontend::HyperFoilAST;
use crate::ui::parse_file;
use flume::{Receiver, Sender};
use hashbrown::HashMap;
use rapier2d::geometry::{ColliderBuilder, ColliderSet};
use rapier2d::prelude::{
    vector, BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodySet,
};
use std::thread::sleep;
use std::time::Duration;

pub struct FlameEngineView<'a> {
    pub rigid_body_set: &'a mut RigidBodySet,
    pub narrow_phase: &'a mut NarrowPhase,
    event_tx: &'a mut Sender<FlameEvent>,
}

impl<'a> FlameEngineView<'a> {
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
            .send(FlameEvent::SwitchToScene(scene_name))
            .unwrap();
    }
    pub fn insert_into_datamap(&self, var: String, val: String) {
        self.event_tx
            .send(FlameEvent::InsertDatamapValue((var, val)))
            .unwrap();
    }
    pub fn set_datamap_value(&self, var: String, val: String) {
        self.event_tx
            .send(FlameEvent::SetDatamapValue((var, val)))
            .unwrap();
    }
    pub fn remove_datamap_value(&self, var: String) {
        self.event_tx
            .send(FlameEvent::RemoveDatamapValue(var))
            .unwrap();
    }
}

#[derive(Clone)]
pub struct Scene {
    pub game_objects: Vec<GameObject>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub narrow_phase_collision: NarrowPhase,
    pub island_manager: IslandManager,
    pub integration_params: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub ui_ast: Option<HyperFoilAST>,
    pub function_map: HashMap<String, fn(&mut FlameEngineView)>,
    pub data_map: HashMap<String, String>,
}
impl Scene {
    pub fn register_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }
    pub fn register_ui(&mut self, ui_file: String) {
        let ui_ast = parse_file(&ui_file);
        self.ui_ast = Some(ui_ast)
    }
}

pub enum FlameEvent {
    SwitchToScene(String),
    SetDatamapValue((String, String)),
    InsertDatamapValue((String, String)),
    RemoveDatamapValue(String),
}

pub struct FlameEngine {
    pub scenes: HashMap<String, Scene>,
    active_scene: Option<Scene>,
    event_rx: Receiver<FlameEvent>,
    event_tx: Sender<FlameEvent>,
    window_width: i32,
    window_height: i32,
}

pub struct FlameConfig {
    pub gravity: f32,
}

impl FlameEngine {
    pub fn new(window_width: i32, window_height: i32) -> Self {

        let (event_tx, event_rx) = flume::bounded(60); //TODO: Set to frame rate

        FlameEngine {
            scenes: HashMap::new(),
            event_tx,
            window_width,
            event_rx,
            active_scene: None,
            window_height,
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
            )
        }
    }

    pub fn start_cycle(&mut self, game_code: fn(&mut Self), config: FlameConfig) {
        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, config.gravity]; // We should scale this instead
                                                    // 850.81
        let mut physics_pipeline = PhysicsPipeline::new();

        loop {
            let active_scene = self.active_scene.as_mut();
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

                let packet = self.event_rx.try_recv();
                match packet {
                    Ok(event) => match event {
                        FlameEvent::SwitchToScene(scene) => {
                            self.set_current_scene(scene);
                        }
                        FlameEvent::SetDatamapValue((var, val)) => {
                            *self
                                .active_scene
                                .as_mut()
                                .unwrap()
                                .data_map
                                .get_mut(&var)
                                .unwrap() = val;
                        }
                        FlameEvent::InsertDatamapValue((var, val)) => {
                            self.active_scene
                                .as_mut()
                                .unwrap()
                                .data_map
                                .insert(var, val);
                        }
                        FlameEvent::RemoveDatamapValue(var) => {
                            self.active_scene.as_mut().unwrap().data_map.remove(&var);
                        }
                    },
                    Err(e) => {
                        // panic!("{}",e); //TODO: Handle
                    }
                }

                {
                    game_code(self);
                }

                let active_scene = self.active_scene.as_mut().unwrap();

                // if active_scene.ui_ast.is_some() {
                // }

                for object in &mut active_scene.game_objects {
                    object.execute(
                        &mut active_scene.rigid_body_set,
                        &mut active_scene.narrow_phase_collision,
                        &mut self.event_tx,
                    );
                }

                // d.clear_background(config.clear_color);
            }

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
            },
        );
        self.scenes.get_mut(&scene_name).unwrap()
    }
}
