use alcubierre::game_object::behaviours::EngineView;
use alcubierre::game_object::graphics::{CircleData, Graphics, GraphicsType};
use alcubierre::physics::{AlcubierreCollider, AlcubierreColliderType};
use alcubierre::Engine;
use rapier2d::prelude::{vector, Ball, RigidBodyBuilder};

fn retry(flame: &mut EngineView) {
    flame.load_scene("Main".to_string());
    println!("RETRY!");
}

pub fn register_fail_scene(mut flame: &mut Engine) {
    let scene = flame.register_scene("Fail".to_string());

    scene.function_map.insert("retry".to_string(), retry);

    // scene.register_ui("examples/pingpong/ui/failed.html".to_string());
}
