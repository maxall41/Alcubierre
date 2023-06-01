use crate::scenes::main::register_main_scene;




use alcubierre::{Engine, EngineConfig};






mod scenes;
mod scripts;



#[tokio::main]
async fn main() {
    let mut engine = Engine::new(640, 480,EngineConfig { gravity: -1.8 });

    register_main_scene(&mut engine);

    // register_second_scene(&mut flame);

    engine.set_current_scene("Main".to_string());

    engine.start_cycle();
    println!("Cycle started");
}
