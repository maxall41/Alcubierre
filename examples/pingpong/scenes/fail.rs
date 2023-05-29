use rapier2d::geometry::ColliderBuilder;
use rapier2d::prelude::{Ball, RigidBodyBuilder, vector};
use raylib::color::Color;
use flame::{FlameEngine, FlameEngineView, Scene};
use flame::game_object::GameObject;
use flame::game_object::graphics::{CircleData, Graphics, GraphicsType, SquareData};
use flame::game_object::physics::PhysicsObject;
use flame::physics::{FlameCollider, FlameColliderType};
use crate::scripts::player::PlayerBehaviour;
use flame::physics::pixels_to_physics_units;
use crate::scripts::ai::AIBehaviour;
use crate::scripts::ball::BallBehaviour;

fn retry(flame: &mut FlameEngineView) {
    flame.load_scene("Main".to_string());
    println!("RETRY!");
}

pub fn register_fail_scene(mut flame: &mut FlameEngine) {
    let scene = flame.register_scene("Fail".to_string());

    scene.function_map.insert("retry".to_string(),retry);

    scene.register_ui("examples/pingpong/ui/failed.html".to_string());


}