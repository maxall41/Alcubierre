use crate::game_object::behaviours::EngineView;
use crate::game_object::GameObject;
use crate::ui::frontend::HyperFoilAST;
use crate::ui::parse_ui_blob;
use hashbrown::HashMap;
use rapier2d::geometry::ColliderSet;
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager,
    MultibodyJointSet, NarrowPhase, RigidBodySet,
};

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
    pub function_map: HashMap<String, fn(&mut EngineView)>,
    pub data_map: HashMap<String, String>,
}
impl Scene {
    pub fn register_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }
    pub fn register_ui(&mut self, blob: &str) {
        let ui_ast = parse_ui_blob(blob);
        self.ui_ast = Some(ui_ast)
    }
}
