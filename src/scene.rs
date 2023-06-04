use crate::game_object::behaviours::EngineView;
use crate::game_object::{GameObject, GameObjectBuilder};
use crate::ui::frontend::HyperFoilAST;
use crate::ui::parse_ui_blob;
use hashbrown::HashMap;
use rapier2d::geometry::{ColliderHandle, ColliderSet};
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager,
    MultibodyJointSet, NarrowPhase, RigidBodySet,
};
use crate::game_object::physics::PhysicsData;

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
    pub(crate) current_game_object_id: u128,
}
impl Scene {
    pub fn register_game_object(&mut self, game_object_builder: GameObjectBuilder) {

        let mut collider_handle : Option<ColliderHandle> = None;
        if game_object_builder.pre_rapier_collider.is_some() {
            if game_object_builder.rigid_body_handle.is_some() {
                let handle = self.collider_set.insert_with_parent(
                    game_object_builder.pre_rapier_collider.unwrap().to_rapier(self.current_game_object_id),
                    game_object_builder.rigid_body_handle.unwrap(),
                    &mut self.rigid_body_set,
                );
                collider_handle = Some(handle.clone());
            } else {
                let handle = self.collider_set.insert(game_object_builder.pre_rapier_collider.unwrap().to_rapier(self.current_game_object_id));
                collider_handle = Some(handle.clone());
            }
        }


        let game_object = GameObject {
            graphics: game_object_builder.graphics,
            behaviours: game_object_builder.behaviours,
            pos_x: game_object_builder.pos_x,
            pos_y: game_object_builder.pos_y,
            physics: PhysicsData {
                collider_handle: collider_handle,
                rigid_body_handle: game_object_builder.rigid_body_handle,
            },
            id: self.current_game_object_id,
        };

        self.game_objects.push(game_object);
        self.current_game_object_id += 1;
    }
    pub fn register_ui(&mut self, blob: &str) {
        let ui_ast = parse_ui_blob(blob);
        self.ui_ast = Some(ui_ast)
    }
}
